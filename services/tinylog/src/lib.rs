mod git;

use constant_time_eq::constant_time_eq;
use secrecy::{ExposeSecret, SecretString};
use worker::*;

use crate::git::{GitAuthor, GitBranch, GitFileMode, GitHubClient, GitHubToken};

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
async fn commit_file(
    client: &GitHubClient,
    path: &str,
    content: &str,
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

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    let url: Url = req.uri().to_string().parse()?;

    let expected_secret = GitHubToken::from(env.secret("SECRET_TOKEN")?.to_string());
    let actual_secret = url
        .query_pairs()
        .find(|(key, _)| key == "token")
        .map(|(_, value)| SecretToken::from(value.to_string()));

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

    let github_token = GitHubToken::from(env.secret("GITHUB_TOKEN")?.to_string());
    let client = GitHubClient::new(
        github_token,
        String::from(REPO_OWNER),
        String::from(REPO_NAME),
    );

    if let Err(err) = commit_file(
        &client,
        "gemini/static/log.gmi",
        "Hello, world!",
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
