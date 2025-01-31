use crate::db::{query_orders, Order};
use crate::templates::HTMLTemplate;
use askama::Template;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    orders: Vec<Order>,
}

pub async fn render(Extension(pool): Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    let orders = query_orders(&pool, 10u32).await.unwrap();

    let template = IndexTemplate { orders };

    HTMLTemplate::from_template(template)
}
