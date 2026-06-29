use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

// ErrorResponse is converted into JSON.
//
// Example:
//
// {
//     "message": "Todo not found"
// }
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub enum ApiError{ 
    NotFound(String), // custom error not found with a message
    BadRequest(String), // custom error bad request with a message
}

// IntoResponse tells Axum how to convert our custom error into an HTTP response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        
        // Match the error variant and choose the correct HTTP status code.
        match self {    
            ApiError::NotFound(message) => {
                (StatusCode::NOT_FOUND, Json(ErrorResponse { message })).into_response()
            }
            ApiError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })).into_response()
            }
        }
    }
}