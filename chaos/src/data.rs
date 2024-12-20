use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn process_data(Json(request): Json<DataRequest>) -> impl IntoResponse {
    // Calculate sums and return response

    let mut string_len = 0;
    let mut int_sum = 0;

    for item in &request.data {
        if item.is_string() {
            if let Some(i) = item.as_str() {
                string_len += i.len();
            }
        } else if item.is_number() {
            if let Some(j) = item.as_i64() {
                int_sum += j;
            }
        }
    }

    let response = DataResponse {
        string_len,
        int_sum,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
pub struct DataRequest {
    // Add any fields here
    pub data: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DataResponse {
    // Add any fields here
    pub string_len: usize,
    pub int_sum: i64,
}
