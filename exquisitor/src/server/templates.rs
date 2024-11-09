use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub struct HTMLTemplate<T>
where
    T: Template,
{
    pub template: T,
    pub code: StatusCode,
}

impl<T> HTMLTemplate<T>
where
    T: Template,
{
    pub fn from_template(template: T) -> Self {
        Self {
            template,
            code: StatusCode::OK,
        }
    }
}

impl<T> IntoResponse for HTMLTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.template.render() {
            Ok(html) => (self.code, Html(html)).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
