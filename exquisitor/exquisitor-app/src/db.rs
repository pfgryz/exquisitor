use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    pub order_id: i64,
    pub name: String,
    pub filepath: String,
    pub status: String,
    pub result_id: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OrderResult {
    pub result_id: i64,
    pub success: bool,
    pub filepath: String,
}

#[derive(Debug)]
pub enum OrderStatus {
    Queued,
    InProgress,
    Done,
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Queued => "QUEUED",
            OrderStatus::InProgress => "IN_PROGRESS",
            OrderStatus::Done => "DONE",
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

pub async fn create_experiment(
    pool: &SqlitePool,
    name: String,
    filepath: String,
    status: OrderStatus
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

// endregion

// region READ

pub async fn get_order_by_id(
    pool: &SqlitePool,
    order_id: i64,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as!(
        Order,
        "SELECT * FROM orders WHERE order_id = $1",
        order_id
    )
    .fetch_optional(pool)
    .await
}

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

pub async fn query_orders(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as!(Order, "SELECT * FROM orders LIMIT ?", limit)
        .fetch_all(pool)
        .await
}

// endregion

// region UPDATE

// endregion