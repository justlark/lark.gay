use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use reqwest::header::HeaderMap;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GITHUB_API_BASE: &str = "https://api.github.com";
const GITHUB_API_VERSION: &str = "2026-03-10";

#[derive(Debug, Clone, Copy, Serialize)]
pub enum GitFileMode {
    #[serde(rename = "100644")]
    Blob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitBlobSha(String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitTreeSha(String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitCommitSha(String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitBranch(Cow<'static, str>);

impl GitBranch {
    pub const fn new(branch: &'static str) -> Self {
        GitBranch(Cow::Borrowed(branch))
    }
}

impl Display for GitBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
pub struct GitAuthor {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct GitHubToken(SecretString);

impl ExposeSecret<str> for GitHubToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl From<String> for GitHubToken {
    fn from(token: String) -> Self {
        GitHubToken(SecretString::from(token))
    }
}

#[derive(Debug)]
pub struct GitHubClient {
    token: GitHubToken,
    client: reqwest::Client,
    owner: String,
    repo: String,
}

impl GitHubClient {
    pub fn new(token: GitHubToken, owner: String, repo: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("justlark")
            .build()
            .expect("Failed to build reqwest client.");

        GitHubClient {
            token,
            client,
            owner,
            repo,
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Accept",
            "application/vnd.github+json"
                .parse()
                .expect("Failed to parse header value."),
        );

        headers.insert(
            "User-Agent",
            self.owner.parse().expect("Failed to parse header value."),
        );

        headers.insert(
            "X-GitHub-Api-Version",
            GITHUB_API_VERSION
                .parse()
                .expect("Failed to parse header value."),
        );

        headers
    }

    pub async fn get_head(&self, branch: &GitBranch) -> anyhow::Result<GitCommitSha> {
        let url = format!(
            "{}/repos/{}/{}/git/ref/heads/{}",
            GITHUB_API_BASE, self.owner, self.repo, branch
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers())
            .bearer_auth(self.token.expose_secret())
            .send()
            .await?;

        #[derive(Deserialize)]
        struct ResponseObject {
            sha: GitCommitSha,
        }

        #[derive(Deserialize)]
        struct Response {
            object: ResponseObject,
        }

        Ok(response.json::<Response>().await.map(|r| r.object.sha)?)
    }

    pub async fn write_blob(&self, content: &str) -> anyhow::Result<GitBlobSha> {
        let url = format!(
            "{}/repos/{}/{}/git/blobs",
            GITHUB_API_BASE, self.owner, self.repo
        );

        let body = json!({
            "content": content,
            "encoding": "utf-8"
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            sha: GitBlobSha,
        }

        Ok(response.json::<Response>().await.map(|r| r.sha)?)
    }

    pub async fn write_tree(
        &self,
        base_sha: &GitCommitSha,
        path: &str,
        mode: GitFileMode,
        blob: &GitBlobSha,
    ) -> anyhow::Result<GitTreeSha> {
        let url = format!(
            "{}/repos/{}/{}/git/trees",
            GITHUB_API_BASE, self.owner, self.repo
        );

        let body = json!({
            "base_tree": base_sha,
            "tree": [
                {
                    "path": path,
                    "mode": mode,
                    "type": "blob",
                    "sha": blob,
                }
            ]
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            sha: GitTreeSha,
        }

        Ok(response.json::<Response>().await.map(|r| r.sha)?)
    }

    pub async fn write_commit(
        &self,
        tree: &GitTreeSha,
        parent: &GitCommitSha,
        message: &str,
        author: &GitAuthor,
    ) -> anyhow::Result<GitCommitSha> {
        let url = format!(
            "{}/repos/{}/{}/git/commits",
            GITHUB_API_BASE, self.owner, self.repo
        );

        let body = json!({
            "message": message,
            "tree": tree,
            "parents": [parent],
            "author": {
                "name": author.name,
                "email": author.email
            }
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            sha: GitCommitSha,
        }

        Ok(response.json::<Response>().await.map(|r| r.sha)?)
    }

    pub async fn update_branch(
        &self,
        branch: &GitBranch,
        sha: &GitCommitSha,
    ) -> anyhow::Result<()> {
        let url = format!(
            "{}/repos/{}/{}/git/refs",
            GITHUB_API_BASE, self.owner, self.repo
        );

        let body = json!({
            "ref": format!("refs/heads/{}", branch),
            "sha": sha,
        });

        self.client
            .post(&url)
            .headers(self.headers())
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}
