mod handlers;

use std::time::Duration;

use handlers::attendance::{get_attendance, insert_attendance, update_attendance};
use handlers::students::get_student;

use anyhow::{self, Context};
use axum::routing::{get, Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_LOCATION")
        .context("Failed to set database url")
        .unwrap();
    // URL Format in case you forget: postgres://postgres:password@ip_addr
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await?;

    let router = Router::new()
        .route("/", get(hello))
        .route("/student", get(get_student))
        .route(
            "/attendance",
            get(get_attendance)
                .post(insert_attendance)
                .put(update_attendance),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;

    axum::serve(listener, router).await?;
    println!("INFO: Running server at port 6969");

    Ok(())
}

async fn hello() -> &'static str {
    include_str!("../assets/html/index.html")
}
