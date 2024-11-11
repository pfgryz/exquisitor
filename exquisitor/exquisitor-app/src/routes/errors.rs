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
            template: ErrorTemplate {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal Server Error".into(),
            },
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

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

    HTMLTemplate {
        template,
        code: StatusCode::NOT_FOUND,
    }
}
