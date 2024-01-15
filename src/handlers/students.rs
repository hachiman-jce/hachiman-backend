use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;

#[derive(Serialize, Deserialize)]
pub struct StudentRequest {
    pub reg_no: Option<i64>,
    pub admission_no: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct StudentResponse {
    pub reg_no: Option<i64>,
    pub admission_no: Option<i32>,
    pub name: String,
    pub dept: String,
    pub class: String,
    pub year: i32,
}

pub async fn get_student(
    Query(params): Query<StudentRequest>,
    State(pool): State<PgPool>,
) -> Result<Json<StudentResponse>, StatusCode> {
    match (params.reg_no, params.admission_no) {
        (Some(reg), Some(adm_no)) => {
            let result = sqlx::query(
                r#"SELECT * FROM student_info WHERE reg_no = $1 AND admission_no = $2;"#,
            )
            .bind(reg)
            .bind(adm_no)
            .fetch_one(&pool)
            .await;

            if let Ok(row) = result {
                return Ok(Json(StudentResponse {
                    reg_no: Some(row.get("reg_no")),
                    admission_no: Some(row.get("admission_no")),
                    name: row.get("name"),
                    class: row.get("class"),
                    dept: row.get("dept"),
                    year: row.get("year"),
                }));
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        (Some(reg), None) => {
            let result = sqlx::query(r#"SELECT * FROM student_info WHERE reg_no = $1;"#)
                .bind(reg)
                .fetch_one(&pool)
                .await;
            if let Ok(row) = result {
                return Ok(Json(StudentResponse {
                    reg_no: Some(row.get("reg_no")),
                    admission_no: Some(row.get("admission_no")),
                    name: row.get("name"),
                    class: row.get("class"),
                    dept: row.get("dept"),
                    year: row.get("year"),
                }));
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        (None, Some(adm_no)) => {
            let result = sqlx::query(r#"SELECT * FROM student_info WHERE admission_no = $1;"#)
                .bind(adm_no)
                .fetch_one(&pool)
                .await;
            if let Ok(row) = result {
                return Ok(Json(StudentResponse {
                    reg_no: Some(row.get("reg_no")),
                    admission_no: Some(row.get("admission_no")),
                    name: row.get("name"),
                    class: row.get("class"),
                    dept: row.get("dept"),
                    year: row.get("year"),
                }));
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        (None, None) => {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}
