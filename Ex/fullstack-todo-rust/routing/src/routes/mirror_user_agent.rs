use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(useragent): TypedHeader<UserAgent>) -> String {
    useragent.to_string()
}
