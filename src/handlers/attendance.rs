// TODO: Make CRUD operations
// TODO: Use Transactions

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::BitVec;
use sqlx::PgPool;
use sqlx::Row;

#[derive(Serialize, Deserialize)]
pub struct GetAttendanceRequest {
    admission_no: i32,
    reg_no: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetAttendanceResponse {
    pub admission_no: i32,
    pub reg_no: i64,
    pub period_data: BitVec,
}

#[derive(Serialize, Deserialize)]
pub struct InsertAttendanceRequest {
    pub admission_no: i32,
    pub reg_no: Option<i64>,
    pub period_data: BitVec,
}

pub struct UpdateAttendanceRequest {
    pub admission_no: i32,
    pub reg_no: Option<i64>,
    pub period_data: BitVec,
}

pub async fn get_attendance(
    Query(params): Query<GetAttendanceRequest>,
    State(pool): State<PgPool>,
) -> Result<Json<GetAttendanceResponse>, StatusCode> {
    let adm_no = params.admission_no;
    let reg = params.reg_no;
    match (adm_no, reg) {
        (adm_no, Some(reg)) => {
            let result = sqlx::query(
                r#"SELECT * FROM attendance_2024_01_15 WHERE reg_no = $1 AND admission_no = $2"#,
            )
            .bind(reg)
            .bind(adm_no)
            .fetch_one(&pool)
            .await;
            if let Ok(row) = result {
                Ok(Json(GetAttendanceResponse {
                    reg_no: row.get("reg_no"),
                    admission_no: row.get("admission_no"),
                    period_data: row.get("period_data"),
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }

        (adm_no, None) => {
            let result =
                sqlx::query(r#"SELECT * FROM attendance_2024_01_15 WHERE admission_no = $1"#)
                    .bind(adm_no)
                    .fetch_one(&pool)
                    .await;
            if let Ok(row) = result {
                Ok(Json(GetAttendanceResponse {
                    reg_no: row.get("reg_no"),
                    admission_no: row.get("admission_no"),
                    period_data: row.get("period_data"),
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn insert_attendance(
    Query(params): Query<InsertAttendanceRequest>,
    State(pool): State<PgPool>,
) -> StatusCode {
    match params {
        InsertAttendanceRequest {
            reg_no: Some(reg),
            admission_no: adm_no,
            period_data,
        } => {
            let result = sqlx::query(r#"INSERT INTO attendance_2024_01_15 (reg_no, admission_no, period_data) VALUES ($1, $2, $3);"#)
                .bind(reg)
                .bind(adm_no)
                .bind(period_data)
                .execute(&pool).await;
            if let Ok(res) = result {
                if res.rows_affected() == 1 {
                    StatusCode::CREATED
                } else {
                    StatusCode::BAD_REQUEST
                }
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        InsertAttendanceRequest { reg_no: None, .. } => StatusCode::NOT_IMPLEMENTED,
    }
}

pub async fn update_attendance(
    Query(params): Query<UpdateAttendanceRequest>,
    State(pool): State<PgPool>,
) -> StatusCode {
    match params {
        UpdateAttendanceRequest {
            reg_no: Some(reg),
            period_data,
            admission_no,
        } => {
            let result = sqlx::query(r#"UPDATE attendance_2024_01_15 SET period_data = $1 WHERE reg_no = $2 AND admission_no = $3;"#).bind(period_data).bind(reg).bind(admission_no).execute(&pool).await;
            if let Ok(row) = result {
                if row.rows_affected() == 1 {
                    StatusCode::OK
                } else {
                    StatusCode::BAD_REQUEST
                }
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        UpdateAttendanceRequest {
            reg_no: None,
            ..
        } => {
            StatusCode::NOT_IMPLEMENTED
        }
    }
}
