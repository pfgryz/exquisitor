use askama::Template;
use axum::response::{IntoResponse};
use crate::server::templates::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    d: String,
}


pub async fn render() -> impl IntoResponse {
    let template = IndexTemplate { d: "Hi".into() };
    HtmlTemplate(template)
}