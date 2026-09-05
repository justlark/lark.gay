mod git;

use chrono::Utc;
use constant_time_eq::constant_time_eq;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use worker::*;

use crate::git::{GitAuthor, GitBranch, GitFileMode, GitHubClient, GitHubToken, GitRef};

const DEFAULT_BRANCH: GitBranch = GitBranch::new("main");
const REPO_OWNER: &str = "justlark";
const REPO_NAME: &str = "lark.gay";
const COMMITTER_NAME: &str = "Lark Space Tinylog Bot";
const COMMITTER_EMAIL: &str = "lark-tinylog[bot]@lark.gay";

#[derive(Debug)]
struct SecretToken(SecretString);

impl From<String> for SecretToken {
    fn from(token: String) -> Self {
        SecretToken(SecretString::from(token))
    }
}

impl ExposeSecret<str> for SecretToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

async fn get_file(client: &GitHubClient, path: &str) -> anyhow::Result<Vec<u8>> {
    let blob_sha = client
        .get_tree(&GitRef::Branch(DEFAULT_BRANCH), path)
        .await?
        .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))?;

    client.get_blob(&blob_sha).await
}
async fn commit_file(
    client: &GitHubClient,
    path: &str,
    content: &[u8],
    message: &str,
) -> anyhow::Result<()> {
    let author = GitAuthor {
        name: COMMITTER_NAME.to_string(),
        email: COMMITTER_EMAIL.to_string(),
    };

    let head_sha = client.get_head(&DEFAULT_BRANCH).await?;
    let blob_sha = client.write_blob(content).await?;
    let tree_sha = client
        .write_tree(&head_sha, path, GitFileMode::Blob, &blob_sha)
        .await?;
    let commit_sha = client
        .write_commit(&tree_sha, &head_sha, message, &author)
        .await?;
    client.update_branch(&DEFAULT_BRANCH, &commit_sha).await?;

    Ok(())
}

async fn append_entry(content: &mut Vec<u8>, message: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M +00:00").to_string();

    content.extend(b"\n\n## ");
    content.extend(timestamp.as_bytes());
    content.extend(b"\n");
    content.extend(message.as_bytes());
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    message: String,
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    if req.method() != reqwest::Method::POST {
        return Ok(http::Response::builder()
            .status(http::StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())?);
    }

    let expected_secret = GitHubToken::from(env.secret("SECRET_TOKEN")?.to_string());
    let actual_secret = req.headers().get("Authorization").map(|header| {
        SecretToken::from(
            header
                .to_str()
                .unwrap_or_default()
                .trim_start_matches("Bearer ")
                .to_string(),
        )
    });

    if !actual_secret
        .map(|actual_secret| {
            constant_time_eq(
                actual_secret.expose_secret().as_bytes(),
                expected_secret.expose_secret().as_bytes(),
            )
        })
        .unwrap_or(false)
    {
        return Ok(http::Response::builder()
            .status(http::StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let message = match worker::Request::try_from(req)
        .expect("Failed request type conversion.")
        .json::<RequestBody>()
        .await
    {
        Err(err) => {
            console_error!("Error parsing request body: {:?}", err);

            return Ok(http::Response::builder()
                .status(http::StatusCode::BAD_REQUEST)
                .body(Body::empty())?);
        }
        Ok(RequestBody { message }) => message,
    };

    let github_token = GitHubToken::from(env.secret("GITHUB_TOKEN")?.to_string());
    let client = GitHubClient::new(
        github_token,
        String::from(REPO_OWNER),
        String::from(REPO_NAME),
    );

    let mut content = match get_file(&client, "gemini/static/log.gmi").await {
        Ok(content) => content,
        Err(err) => {
            console_error!("Error getting file: {:?}", err);

            return Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())?);
        }
    };

    append_entry(&mut content, &message).await;

    if let Err(err) = commit_file(
        &client,
        "gemini/static/log.gmi",
        &content,
        "Update capsule tinylog",
    )
    .await
    {
        console_error!("Error committing file: {:?}", err);

        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?);
    }

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::empty())?)
}
