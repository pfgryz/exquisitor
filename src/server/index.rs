use askama::Template;
use axum::Extension;
use axum::response::{IntoResponse};
use sqlx::SqlitePool;
use crate::server::db::{query_experiments, Experiment};
use crate::server::templates::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    experiments: Vec<Experiment>
}


pub async fn render(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let experiments = query_experiments(&pool, 10u32)
        .await
        .unwrap(); // @TODO: add error handling
    let template = IndexTemplate {
        experiments
    };
    HtmlTemplate(template)
}