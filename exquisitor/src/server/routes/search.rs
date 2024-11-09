use crate::errors::InternalServerError;
use crate::server::db;
use crate::server::templates::HTMLTemplate;
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
    Extension(pool): Extension<SqlitePool>,
) -> Response {
    let experiment_id = params.query.parse::<i64>().unwrap_or(0);

    let experiment =
        match db::get_experiment_by_id_or_name(&pool, experiment_id, params.query.as_str())
            .await
            .map_err(|e| InternalServerError::DatabaseError(e))
        {
            Ok(experiment) => experiment,
            Err(e) => return e.into_response(),
        };

    let experiment = match experiment {
        Some(experiment) => experiment,
        None => {
            return HTMLTemplate {
                template: SearchTemplate {},
                code: StatusCode::NOT_FOUND,
            }
            .into_response()
        }
    };

    Redirect::to(format!("/experiment/{}", experiment.experiment_id).as_str()).into_response()
}
