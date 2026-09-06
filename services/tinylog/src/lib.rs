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

async fn get_file(client: &GitHubClient, path: &str) -> anyhow::Result<String> {
    let blob_sha = client
        .get_tree(&GitRef::Branch(DEFAULT_BRANCH), path)
        .await?
        .ok_or_else(|| anyhow::anyhow!("File not found: {}", path))?;

    client.get_blob(&blob_sha).await
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

async fn add_entry(content: &mut String, message: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M").to_string();

    while content.ends_with('\n') {
        content.pop();
    }

    let mut split_index = 0;

    for line in content.lines() {
        if line.starts_with("## ") {
            break;
        }

        // Add one for the newline character.
        split_index += line.len() + 1;
    }

    let entries_after = content.split_off(split_index - 1);

    content.push_str("\n## ");
    content.push_str(timestamp.as_str());
    content.push('\n');
    content.push_str(message);
    content.push('\n');

    content.push_str(entries_after.as_str());
}

fn set_cors_headers(headers: &mut http::HeaderMap) {
    headers.insert(
        "Access-Control-Allow-Origin",
        "https://lark.gay"
            .parse()
            .expect("Failed to parse header value."),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "POST, OPTIONS"
            .parse()
            .expect("Failed to parse header value."),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        "Authorization, Content-Type"
            .parse()
            .expect("Failed to parse header value."),
    );
}

fn empty_response(status: http::StatusCode) -> Result<HttpResponse> {
    let mut builder = http::Response::builder().status(status);
    set_cors_headers(builder.headers_mut().unwrap());
    Ok(builder.body(Body::empty())?)
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    message: String,
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    if req.method() == reqwest::Method::OPTIONS {
        return empty_response(http::StatusCode::OK);
    }

    if req.method() != reqwest::Method::POST {
        return empty_response(http::StatusCode::METHOD_NOT_ALLOWED);
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
        return empty_response(http::StatusCode::UNAUTHORIZED);
    }

    let message = match worker::Request::try_from(req)
        .expect("Failed request type conversion.")
        .json::<RequestBody>()
        .await
    {
        Err(err) => {
            console_error!("Error parsing request body: {:?}", err);

            return empty_response(http::StatusCode::BAD_REQUEST);
        }
        Ok(RequestBody { message }) => message,
    };

    let github_token = GitHubToken::from(env.secret("GITHUB_TOKEN")?.to_string());
    let client = GitHubClient::new(
        github_token,
        String::from(REPO_OWNER),
        String::from(REPO_NAME),
    );

    let mut content = match get_file(&client, "gemini/static/log/index.gmi").await {
        Ok(content) => content,
        Err(err) => {
            console_error!("Error getting file: {:?}", err);

            return empty_response(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    add_entry(&mut content, &message).await;

    if let Err(err) = commit_file(
        &client,
        "gemini/static/log/index.gmi",
        &content,
        "Update capsule tinylog",
    )
    .await
    {
        console_error!("Error committing file: {:?}", err);

        return empty_response(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    empty_response(http::StatusCode::OK)
}
