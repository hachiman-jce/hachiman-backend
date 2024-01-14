use std::time::Duration;

use anyhow::{self, Context};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::Row;

#[derive(Serialize, Deserialize)]
struct Student {
    name: String,
    reg_no: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DB_LOC").context("Failed to set database url").unwrap();
    // URL Format in case you forget: postgres://postgres:password@ip_addr
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await?;


    let router = Router::new()
        .route("/", get(hello))
        .route("/student", post(insert_student))
        .route("/student", get(student))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn hello() -> &'static str {
    "hello, world!"
}

async fn insert_student(params: Query<Student>, State(pool): State<PgPool>) -> StatusCode {
    let result = sqlx::query(r#"INSERT INTO students (name, regno) VALUES($1, $2)"#)
        .bind(&params.name)
        .bind(params.reg_no)
        .execute(&pool)
        .await;

    if let Ok(query_result) = result {
        if query_result.rows_affected() == 1 {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn student(params: Query<Student>, State(pool): State<PgPool>) -> Result<String, StatusCode> {
    let result = sqlx::query(r#"SELECT * FROM students WHERE regno = $1"#)
        .bind(params.reg_no)
        .fetch_one(&pool)
        .await;
    if let Ok(row) = result {
        Ok(row.get("name"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
