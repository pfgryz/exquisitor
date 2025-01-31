//! Module for database operations
//!
//! Defines tables and provides CRUD operations for managing data about orders and results.

use sqlx::sqlite::SqliteRow;
use sqlx::{Error, SqlitePool};

///
///
#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    /// Identifier of order
    pub order_id: i64,

    /// Name of the order
    pub name: String,

    /// Filepath for input data file with sequences
    pub filepath: String,

    /// Status of the order
    pub status: String,

    /// Unique identifier for the result (optional).
    pub result_id: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OrderResult {
    /// Identifier of order result
    #[allow(dead_code)] // Note: this field is not used in solution, but is present in database
    pub result_id: i64,

    /// Indicates the success status (1 for success, 0 for failure)
    #[allow(dead_code)] // Note: this field is not used in solution, but is present in database
    pub success: i64,

    /// Filepath for output data file (optional)
    pub filepath: Option<String>,
}

/// Possible statuses of order
#[derive(Debug, PartialEq, Eq)]
pub enum OrderStatus {
    /// Order in queue, waiting for execution
    Queued,

    /// Execution of order in progress
    InProgress,

    /// Order executed successfully
    Done,

    /// Order executed with error
    Failed,
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Queued => "QUEUED",
            OrderStatus::InProgress => "IN_PROGRESS",
            OrderStatus::Done => "DONE",
            OrderStatus::Failed => "FAILED",
        }
    }
}

impl TryFrom<String> for OrderStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "QUEUED" => Ok(OrderStatus::Queued),
            "IN_PROGRESS" => Ok(OrderStatus::InProgress),
            "DONE" => Ok(OrderStatus::Done),
            "FAILED" => Ok(OrderStatus::Failed),
            _ => Err(format!("Invalid status value: {}", value)),
        }
    }
}

impl From<OrderStatus> for String {
    fn from(status: OrderStatus) -> Self {
        status.as_str().to_string()
    }
}

// region CREATE

/// Creates new order in database
pub async fn create_order(
    pool: &SqlitePool,
    name: String,
    filepath: String,
    status: OrderStatus,
) -> Result<Option<i64>, sqlx::Error> {
    let status = status.as_str();

    let order = sqlx::query!(
        "INSERT INTO orders (name, filepath, status) VALUES ($1, $2, $3) RETURNING order_id",
        name,
        filepath,
        status
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(order.order_id))
}

/// Crates new order result in database
pub async fn create_result(
    pool: &SqlitePool,
    success: bool,
    filepath: Option<String>,
) -> Result<Option<i64>, sqlx::Error> {
    let filepath = filepath.unwrap_or("NULL".into());
    let result = sqlx::query!(
        "INSERT INTO results (success, filepath) VALUES ($1, $2) RETURNING result_id",
        success,
        filepath
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(result.result_id))
}

// endregion

// region READ

/// Get the order by order identifier
pub async fn get_order_by_id(
    pool: &SqlitePool,
    order_id: i64,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as!(Order, "SELECT * FROM orders WHERE order_id = $1", order_id)
        .fetch_optional(pool)
        .await
}

/// Get order by order identifier or order name
pub async fn get_order_by_id_or_name(
    pool: &SqlitePool,
    order_id: i64,
    name: &str,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as!(
        Order,
        "SELECT * FROM orders WHERE order_id = $1 OR name = $2",
        order_id,
        name
    )
    .fetch_optional(pool)
    .await
}

/// Get first `limit` orders
pub async fn query_orders(pool: &SqlitePool, limit: u32) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as!(Order, "SELECT * FROM orders LIMIT ?", limit)
        .fetch_all(pool)
        .await
}

/// Get all orders with given status
pub async fn query_orders_by_status(
    pool: &SqlitePool,
    status: OrderStatus,
    limit: Option<u32>,
) -> Result<Vec<Order>, sqlx::Error> {
    let status = String::from(status);

    if let Some(limit) = limit {
        sqlx::query_as!(
            Order,
            "SELECT * FROM orders WHERE status = $1 LIMIT $2",
            status,
            limit
        )
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as!(Order, "SELECT * FROM orders WHERE status = ?", status)
            .fetch_all(pool)
            .await
    }
}

/// Get result by result identifier
pub async fn get_result_by_id(
    pool: &SqlitePool,
    result_id: i64,
) -> Result<Option<OrderResult>, sqlx::Error> {
    sqlx::query_as!(
        OrderResult,
        "SELECT * FROM results WHERE result_id = $1",
        result_id
    )
    .fetch_optional(pool)
    .await
}

// endregion

// region UPDATE

/// Update the status of the order
pub async fn update_order_status(
    pool: &SqlitePool,
    order_id: i64,
    status: OrderStatus,
) -> Result<Vec<SqliteRow>, Error> {
    let status = String::from(status);
    sqlx::query!(
        "UPDATE orders SET status = $1 WHERE order_id = $2",
        status,
        order_id
    )
    .fetch_all(pool)
    .await
}

/// Update the result id for the order
pub async fn update_order_result(
    pool: &SqlitePool,
    order_id: i64,
    result_id: i64,
) -> Result<Vec<SqliteRow>, Error> {
    sqlx::query!(
        "UPDATE orders SET result_id = $1 WHERE order_id = $2",
        result_id,
        order_id
    )
    .fetch_all(pool)
    .await
}

// endregion
