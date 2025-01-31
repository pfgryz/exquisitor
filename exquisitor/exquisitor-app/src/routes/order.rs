use crate::db;
use crate::db::{get_result_by_id, Order, OrderResult, OrderStatus};
use crate::routes::errors::{create_code_response, handle_not_found, InternalServerError};
use crate::templates::HTMLTemplate;
use crate::Arc;
use askama::Template;
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Extension;
use sqlx::{Error, SqlitePool};
use std::future::Future;
use std::io;
use std::io::Write;
use tempfile::{Builder, NamedTempFile};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Template)]
#[template(path = "order.html")]
struct OrderTemplate {
    order: Order,
}

async fn get_order(id: i64, pool: &Arc<SqlitePool>) -> Result<Order, Response> {
    let order = match db::get_order_by_id(&pool, id)
        .await
        .map_err(|e| InternalServerError::DatabaseError(e))
    {
        Ok(order) => order,
        Err(e) => return Err(e.into_response()),
    };
    let order = match order {
        Some(order) => order,
        _ => return Err(handle_not_found().await.into_response()),
    };
    Ok(order)
}

pub async fn render(Path(id): Path<i64>, Extension(pool): Extension<Arc<SqlitePool>>) -> Response {
    let order = match get_order(id, &pool).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let template = HTMLTemplate {
        template: OrderTemplate { order },
        code: StatusCode::OK,
    };
    template.into_response()
}

#[derive(Template)]
#[template(path = "order_add.html")]
struct OrderAddTemplate;

pub async fn add_form() -> Response {
    let template = HTMLTemplate {
        template: OrderAddTemplate {},
        code: StatusCode::OK,
    };

    template.into_response()
}

pub async fn create_file(prefix: &str, suffix: &str, directory: &str) -> io::Result<NamedTempFile> {
    tokio::fs::create_dir_all(directory).await?;

    let file = Builder::new()
        .prefix(prefix)
        .suffix(suffix)
        .rand_bytes(5)
        .keep(true)
        .tempfile_in(directory)?;

    Ok(file)
}

pub async fn add_submit(
    Extension(pool): Extension<Arc<SqlitePool>>,
    mut multipart: Multipart,
) -> Response {
    let mut name = String::new();
    let mut file_data = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();

        if field_name == "name" {
            if let Ok(n) = field.text().await {
                name = n;
            } else {
                return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                    .into_response();
            }
        } else if field_name == "file" {
            let content = field.bytes().await;

            if let Ok(bytes) = content {
                file_data.extend_from_slice(&bytes);
            } else {
                return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                    .into_response();
            }
        }
    }

    let mut file = create_file("input-", ".fasta", "exquisitor-fs").await;
    if let Err(e) = file {
        return create_code_response(StatusCode::BAD_REQUEST, "Bad Request").into_response();
    }
    let mut file = file.unwrap();
    let path = { file.path().to_string_lossy().to_string() };
    if let Err(e) = file.as_file_mut().write_all(&file_data) {
        return create_code_response(StatusCode::BAD_REQUEST, "Bad Request").into_response();
    }

    db::create_order(&pool, name.into(), path, OrderStatus::Queued)
        .await
        .expect("");

    return create_code_response(StatusCode::CREATED, "Created").into_response();
}

pub async fn download(
    Path((id, kind)): Path<(i64, String)>,
    Extension(pool): Extension<Arc<SqlitePool>>,
) -> Response {
    let order = match get_order(id, &pool).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let kind = kind.to_lowercase();

    let path = match kind.as_str() {
        "input" => order.filepath,
        "output" => {
            if order.status == OrderStatus::Done.as_str() || order.result_id.is_none() {
                return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                    .into_response();
            }

            match get_result_by_id(&pool, order.result_id.unwrap()).await {
                Ok(result) => {
                    if let Some(result) = result {
                        if let Some(filepath) = result.filepath {
                            filepath
                        } else {
                            return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                                .into_response();
                        }
                    } else {
                        return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                            .into_response();
                    }
                }
                Err(_) => {
                    return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                        .into_response()
                }
            }
        }
        _ => {
            return create_code_response(StatusCode::BAD_REQUEST, "Bad Request").into_response();
        }
    };

    match File::open(path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(err) = file.read_to_end(&mut buffer).await {
                return create_code_response(StatusCode::BAD_REQUEST, "Bad Request")
                    .into_response();
            }

            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&format!("attachment; filename=\"{}\"", kind)).unwrap(),
            );

            (headers, buffer).into_response()
        }
        Err(_) => {
            return create_code_response(StatusCode::BAD_REQUEST, "Bad Request").into_response()
        }
    }
}
