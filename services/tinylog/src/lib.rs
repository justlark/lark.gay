mod git;

use worker::*;

use crate::git::{GitAuthor, GitBranch, GitFileMode, GitHubClient, GitHubToken};

const DEFAULT_BRANCH: GitBranch = GitBranch::new("main");
const REPO_OWNER: &str = "justlark";
const REPO_NAME: &str = "lark.gay";
const COMMITTER_NAME: &str = "Lark Space Tinylog Bot";
const COMMITTER_EMAIL: &str = "lark-tinylog[bot]@lark.gay";

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
async fn fetch(_req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    let token = GitHubToken::from(env.secret("GITHUB_TOKEN")?.to_string());
    let client = GitHubClient::new(token, String::from(REPO_OWNER), String::from(REPO_NAME));

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
