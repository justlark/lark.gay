pub fn set_cors_headers(headers: &mut http::HeaderMap) {
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
