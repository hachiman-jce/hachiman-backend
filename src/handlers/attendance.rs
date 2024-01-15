// Make CRUD operations
use axum::extract::{Query, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use sqlx::types::BitVec;

#[derive(Serialize, Deserialize)]
pub struct GetAttendanceRequest {
    admission_no: i64,
    reg_no: Option<i64>,
}

// #[derive(Serialize, Deserialize)]
// pub struct GetAttendanceResponse {
//     pub admission_no: i64,
//     pub reg_no: i64,
//     pub period_data: BitVec,
// }

// pub async fn get_attendance(
//     Query(params): Query<GetAttendanceRequest>,
//     State(pool): State<PgPool>,
// ) -> Result<GetAttendanceResponse, StatusCode> {
//     let adm_no = params.admission_no;
//     let reg = params.reg_no;
//     match (adm_no, reg) {
//         (adm_no, Some(reg)) => {
//             let result = sqlx::query(
//                 r#"SELECT * FROM attendance_2024_01_15 WHERE reg_no = $1 AND admission_no = $2"#,
//             )
//             .bind(reg)
//             .bind(adm_no)
//             .fetch_one(&pool)
//             .await;
//             if let Ok(row) = result {
//                 Ok(GetAttendanceResponse {
//                     reg_no: row.get("reg_no"),
//                     admission_no: row.get("admission_no"),
//                     period_data: row.get("period_data"),
//                 })
//             } else {
//                 Err(StatusCode::INTERNAL_SERVER_ERROR)
//             }
//         },
//
//         (adm_no, None) => {
//             let result = sqlx::query(r#"SELECT * FROM attendance_2024_01_15 WHERE admission_no = $1"#).bind(adm_no).fetch_one(&pool).await;
//             if let Ok(row) = result {
//                 let period_data: BitVec = row.get("period_data");
//                 let period_data = period_data.to_bytes()
//                 Ok(GetAttendanceResponse {
//                     reg_no: row.get("reg_no"),
//                     admission_no: row.get("admission_no"),
//                     period_data,
//                 })
//             } else {
//                 Err(StatusCode::INTERNAL_SERVER_ERROR)
//             }
//         }
//     }
// }
