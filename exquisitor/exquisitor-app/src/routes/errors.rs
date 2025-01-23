use crate::templates::HTMLTemplate;
use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;

#[derive(Debug)]
pub enum InternalServerError {
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

pub fn create_code_response(code: StatusCode, message: &str) -> impl IntoResponse {
    let template = CodeTemplate {
        code,
        message: message.to_string(),
    };

    HTMLTemplate { template, code }
}

pub async fn handle_not_found() -> impl IntoResponse {
    create_code_response(StatusCode::NOT_FOUND, "Not Found")
}
