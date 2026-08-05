use worker::*;

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    let kv = env.kv("KV")?;
    let r2 = env.bucket("R2")?;
    let slug = req.uri().path().trim_matches('/');

    // A link can either redirect to another URL or serve a file from R2. The target URL takes
    // precedence if both are set.
    if let Some(dest_url) = kv.get(&format!("slug:{}:link", slug)).text().await? {
        return Ok(http::Response::builder()
            .status(http::StatusCode::FOUND)
            .header("Location", dest_url)
            .body(Body::empty())?);
    };

    match kv.get(&format!("slug:{}:file", slug)).text().await? {
        Some(filename) => {
            let object = match r2.get(&filename).execute().await? {
                Some(object) => object,
                None => {
                    return Ok(http::Response::builder()
                        .status(http::StatusCode::NOT_FOUND)
                        .body(Body::empty())?);
                }
            };

            let response_body = object
                .body()
                .expect("Failed to read object body")
                .response_body()?;

            HttpResponse::try_from(Response::from_body(response_body)?)
        }
        None => Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(Body::empty())?),
    }
}
