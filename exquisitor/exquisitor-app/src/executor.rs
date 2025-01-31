//! Orders' executor

use crate::db::{
    create_result, query_orders_by_status, update_order_result, update_order_status, OrderStatus,
};
use crate::routes::order::create_file;
use sqlx::SqlitePool;
use std::env;
use std::path::PathBuf;
use std::process::Output;
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command;
use tracing::{debug, info};

/// Executes the orders and saves results
///
/// On startup, moves all in-progress orders back to the queue.
/// Periodically checks for orders in the queue and processes them.
/// Saves results to the database.
pub async fn executor_task(pool: Arc<SqlitePool>) {
    let orders = query_orders_by_status(&pool, OrderStatus::InProgress, None)
        .await
        .expect("Failed to query orders by status");

    for order in orders.iter() {
        update_order_status(&pool, order.order_id, OrderStatus::Queued)
            .await
            .expect("Failed to update order status");
    }

    loop {
        let order = query_orders_by_status(&pool, OrderStatus::Queued, Some(1))
            .await
            .expect("Failed to query order");

        if order.len() == 0 {
            tokio::time::sleep(Duration::from_secs(10)).await;
            continue;
        }

        let order = &order[0];

        update_order_status(&pool, order.order_id, OrderStatus::InProgress)
            .await
            .expect("Failed to update order status");

        let filename = create_file("output-", ".txt", "exquisitor-fs")
            .await
            .expect("Failed to create directory")
            .path()
            .to_string_lossy()
            .to_string();

        let result = run_exquisitor_analysis(order.filepath.as_str(), &filename).await;

        let status = if result.is_ok() && result.unwrap() {
            OrderStatus::Done
        } else {
            OrderStatus::Failed
        };
        let success = status == OrderStatus::Done;

        update_order_status(&pool, order.order_id, status)
            .await
            .expect("Failed to update order status");

        let result_id = create_result(&pool, success, if success { Some(filename) } else { None })
            .await
            .expect("Failed to create result");

        if result_id.is_some() {
            update_order_result(&pool, order.order_id, result_id.unwrap())
                .await
                .expect("Failed to update order result");
        }
    }
}

/// Runs the ordered analysis
async fn run_exquisitor_analysis(
    input_filename: &str,
    output_filename: &str,
) -> Result<bool, ()> {
    let mut program = env::current_exe().map_err(|_| ())?;
    if let Some(extension) = program.extension() {
        let filename = format!("exquisitor-cli.{}", extension.to_string_lossy());
        program.set_file_name(filename);
    } else {
        program.set_file_name("exquisitor-cli");
    }

    let blast = get_env("BLAST").map_err(|_| ())?;
    let blast_db = get_env("BLASTDB").map_err(|_| ())?;
    let model = get_env("MODEL").map_err(|_| ())?;

    let args = vec![
        "run",
        "--input",
        input_filename,
        "--output",
        output_filename,
        "--blast",
        blast.as_str(),
        "--blast-db",
        blast_db.as_str(),
        "--pipeline",
        "neural",
        "--clustering",
        "k-medoid",
        "--k",
        "100",
        "--model",
        model.as_str(),
    ];
    info!("Running CLI! {}", program.to_string_lossy().to_string());
    debug!("Args: {:?}", args);
    run_exquisitor_cli(program, args).await
}

fn get_env(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(key).map_err(|e| e.into())
}

async fn run_exquisitor_cli(program: PathBuf, arguments: Vec<&str>) -> Result<bool, ()> {
    let output: Output = Command::new(program)
        .args(&arguments)
        .output()
        .await
        .map_err(|_| ())?;

    Ok(output.status.success())
}
