use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::server::templates::HTMLTemplate;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    code: StatusCode,
    message: String,
}

pub async fn handle_not_found() -> impl IntoResponse {
    let template = ErrorTemplate {
        code: StatusCode::NOT_FOUND,
        message: "Not found".into(),
    };

    HTMLTemplate { template, code: StatusCode::NOT_FOUND }
}