use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Experiment {
    pub experiment_id: i64,
    pub name: String,
    pub filepath: String,
    pub status: String,
    pub result_id: Option<i64>,
}

pub async fn get_experiment_by_id(pool: &SqlitePool, experiment_id: i64) -> Result<Option<Experiment>, sqlx::Error> {
    sqlx::query_as!(
        Experiment,
        "SELECT * FROM experiments WHERE experiment_id = $1",
        experiment_id
    )
        .fetch_optional(pool)
        .await
}

pub async fn get_experiment_by_id_or_name(pool: &SqlitePool, experiment_id: i64, name: &str) -> Result<Option<Experiment>, sqlx::Error> {
    sqlx::query_as!(
        Experiment,
        "SELECT * FROM experiments WHERE experiment_id = $1 OR name = $2",
        experiment_id,
        name
    )
        .fetch_optional(pool)
        .await
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