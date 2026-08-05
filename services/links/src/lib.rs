use worker::*;

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<HttpResponse> {
    let slug = req.uri().path().trim_matches('/').to_string();

    todo!()
}

