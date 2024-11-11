use crate::db;
use crate::db::Experiment;
use crate::routes::errors::{handle_not_found, InternalServerError};
use crate::templates::HTMLTemplate;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "experiment.html")]
struct ExperimentTemplate {
    experiment: Experiment,
}

pub async fn render(Path(id): Path<i64>, Extension(pool): Extension<SqlitePool>) -> Response {
    let experiment = match db::get_experiment_by_id(&pool, id)
        .await
        .map_err(|e| InternalServerError::DatabaseError(e))
    {
        Ok(experiment) => experiment,
        Err(e) => return e.into_response(),
    };

    let experiment = match experiment {
        Some(experiment) => experiment,
        _ => return handle_not_found().await.into_response(),
    };

    let template = HTMLTemplate {
        template: ExperimentTemplate { experiment },
        code: StatusCode::OK,
    };
    template.into_response()
}
