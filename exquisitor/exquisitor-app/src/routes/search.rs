use std::sync::Arc;
use crate::db;
use crate::routes::errors::InternalServerError;
use crate::templates::HTMLTemplate;
use askama::Template;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    query: String,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate;

pub async fn render(
    Query(params): Query<SearchParams>,
    Extension(pool): Extension<Arc<SqlitePool>>,
) -> Response {
    let order_id = params.query.parse::<i64>().unwrap_or(0);

    let order = match db::get_order_by_id_or_name(&pool, order_id, params.query.as_str())
        .await
        .map_err(|e| InternalServerError::DatabaseError(e))
    {
        Ok(order) => order,
        Err(e) => return e.into_response(),
    };

    let order = match order {
        Some(order) => order,
        None => {
            return HTMLTemplate {
                template: SearchTemplate {},
                code: StatusCode::NOT_FOUND,
            }
            .into_response()
        }
    };

    Redirect::to(format!("/order/{}", order.order_id).as_str()).into_response()
}
