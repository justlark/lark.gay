use worker::*;

fn not_found() -> Result<HttpResponse> {
    Ok(http::Response::builder()
        .status(http::StatusCode::NOT_FOUND)
        .body(Body::empty())?)
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    let kv = env.kv("KV")?;
    let r2 = env.bucket("R2")?;

    let slug = req.uri().path().trim_matches('/');

    let code = match req.uri().query() {
        None => {
            return not_found();
        }
        Some(query) => match query.strip_prefix("code=") {
            Some(code) => code,
            None => {
                return not_found();
            }
        },
    };

    let filename = match kv.get(&format!("slug:{}:file", slug)).text().await? {
        Some(filename) => filename,
        None => {
            return not_found();
        }
    };

    let code_exists = kv
        .get(&format!("slug:{}:code:{}", slug, code))
        .text()
        .await?
        .is_some();

    if !code_exists {
        return not_found();
    }

    kv.delete(&format!("slug:{}:code:{}", slug, code)).await?;

    let object = match r2.get(&filename).execute().await? {
        Some(object) => object,
        None => {
            return not_found();
        }
    };

    let response_body = object
        .body()
        .expect("Failed to read object body")
        .response_body()?;

    HttpResponse::try_from(Response::from_body(response_body)?)
}

