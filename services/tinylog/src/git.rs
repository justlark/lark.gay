use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use base64::engine::{Engine, general_purpose::STANDARD as BASE64};
use reqwest::header::HeaderMap;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GITHUB_API_BASE: &str = "https://api.github.com";
const GITHUB_API_VERSION: &str = "2026-03-10";

#[derive(Debug, Clone, Copy)]
enum MediaType {
    Json,
    Raw,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum GitFileMode {
    #[serde(rename = "100644")]
    Blob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitBlobSha(String);

impl Display for GitBlobSha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GitRef {
    Branch(GitBranch),
    Commit(GitCommitSha),
}

impl Display for GitRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitRef::Branch(branch) => write!(f, "refs/heads/{}", branch),
            GitRef::Commit(commit) => write!(f, "{}", commit.0),
        }
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

    fn headers(&self, media_type: MediaType) -> HeaderMap {
        let mut headers = HeaderMap::new();

        match media_type {
            MediaType::Json => {
                headers.insert(
                    "Accept",
                    "application/vnd.github+json"
                        .parse()
                        .expect("Failed to parse header value."),
                );
            }
            MediaType::Raw => {
                headers.insert(
                    "Accept",
                    "application/vnd.github.raw+json"
                        .parse()
                        .expect("Failed to parse header value."),
                );
            }
        }

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
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

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

    pub async fn get_blob(&self, sha: &GitBlobSha) -> anyhow::Result<Vec<u8>> {
        let url = format!(
            "{}/repos/{}/{}/git/blobs/{}",
            GITHUB_API_BASE, self.owner, self.repo, sha
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers(MediaType::Raw))
            .bearer_auth(self.token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        Ok(response.bytes().await?.to_vec())
    }

    pub async fn write_blob(&self, content: &[u8]) -> anyhow::Result<GitBlobSha> {
        let url = format!(
            "{}/repos/{}/{}/git/blobs",
            GITHUB_API_BASE, self.owner, self.repo
        );

        let body = json!({
            "content": BASE64.encode(content),
            "encoding": "base64"
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        #[derive(Deserialize)]
        struct Response {
            sha: GitBlobSha,
        }

        Ok(response.json::<Response>().await.map(|r| r.sha)?)
    }

    pub async fn get_tree(
        &self,
        ref_name: &GitRef,
        path: &str,
    ) -> anyhow::Result<Option<GitBlobSha>> {
        let url = format!(
            "{}/repos/{}/{}/git/trees/{}?recursive=1",
            GITHUB_API_BASE, self.owner, self.repo, ref_name,
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        #[derive(Deserialize)]
        struct TreeResponse {
            path: String,
            sha: GitBlobSha,
        }

        #[derive(Deserialize)]
        struct Response {
            tree: Vec<TreeResponse>,
        }

        Ok(response
            .json::<Response>()
            .await?
            .tree
            .into_iter()
            .find_map(|item| {
                if item.path == path {
                    Some(item.sha)
                } else {
                    None
                }
            }))
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
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

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
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

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
            "{}/repos/{}/{}/git/refs/heads/{}",
            GITHUB_API_BASE, self.owner, self.repo, branch
        );

        let body = json!({
            "sha": sha,
        });

        self.client
            .patch(&url)
            .headers(self.headers(MediaType::Json))
            .bearer_auth(self.token.expose_secret())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
