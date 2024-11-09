use crate::server::db::{query_experiments, Experiment};
use crate::server::templates::HTMLTemplate;
use askama::Template;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    experiments: Vec<Experiment>,
}

pub async fn render(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let experiments = query_experiments(&pool, 10u32).await.unwrap(); // @TODO: add error handling

    let template = IndexTemplate { experiments };

    HTMLTemplate::from_template(template)
}
