use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Experiment {
    pub experiment_id: i64,
    pub name: String,
    pub filepath: String,
    pub status: String,
    pub result_id: Option<i64>,
}

pub async fn query_experiments(pool: &SqlitePool, limit: u32) -> Result<Vec<Experiment>, sqlx::Error> {
    sqlx::query_as!(
        Experiment,
        "SELECT * FROM experiments LIMIT ?",
        limit
    )
        .fetch_all(pool)
        .await
}