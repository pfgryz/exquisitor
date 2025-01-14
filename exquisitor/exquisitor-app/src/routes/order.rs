use crate::db;
use crate::db::Order;
use crate::routes::errors::{handle_not_found, InternalServerError};
use crate::templates::HTMLTemplate;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "order.html")]
struct OrderTemplate {
    order: Order,
}

pub async fn render(Path(id): Path<i64>, Extension(pool): Extension<SqlitePool>) -> Response {
    let order = match db::get_order_by_id(&pool, id)
        .await
        .map_err(|e| InternalServerError::DatabaseError(e))
    {
        Ok(order) => order,
        Err(e) => return e.into_response(),
    };

    let order = match order {
        Some(order) => order,
        _ => return handle_not_found().await.into_response(),
    };

    let template = HTMLTemplate {
        template: OrderTemplate{ order },
        code: StatusCode::OK,
    };
    template.into_response()
}
