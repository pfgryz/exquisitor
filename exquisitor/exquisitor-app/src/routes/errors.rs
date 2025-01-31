//! Defines server error types and handles rendering of error pages.

use crate::templates::HTMLTemplate;
use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;

/// Wraps application errors in an internal server error.
#[derive(Debug)]
pub(crate) enum InternalServerError {
    DatabaseError(sqlx::Error),
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        let (name, details) = match self {
            InternalServerError::DatabaseError(e) => ("Database Error", e.to_string()),
        };
        error!("Error: {}\n\t{}", name, details);

        HTMLTemplate {
            template: CodeTemplate {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal Server Error".into(),
            },
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

#[derive(Template)]
#[template(path = "code.html")]
struct CodeTemplate {
    code: StatusCode,
    message: String,
}

/// Creates a server response with a status code and message.
pub(crate) fn create_code_response(code: StatusCode, message: &str) -> impl IntoResponse {
    let template = CodeTemplate {
        code,
        message: message.to_string(),
    };

    HTMLTemplate { template, code }
}

/// Crates a server response for not found resource
pub(crate) async fn handle_not_found() -> impl IntoResponse {
    create_code_response(StatusCode::NOT_FOUND, "Not Found")
}
