use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_val = headers.get("user-agent").unwrap();
    message_val.to_str().unwrap().to_owned()
}
