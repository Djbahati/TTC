use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    Unauthorized,
    BadRequest(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    fn extract_status(error: AppError) -> StatusCode {
        let response = error.into_response();
        response.status()
    }

    #[test]
    fn not_found_returns_404() {
        assert_eq!(extract_status(AppError::NotFound), StatusCode::NOT_FOUND);
    }

    #[test]
    fn unauthorized_returns_401() {
        assert_eq!(extract_status(AppError::Unauthorized), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn bad_request_returns_400() {
        assert_eq!(
            extract_status(AppError::BadRequest("invalid input".to_string())),
            StatusCode::BAD_REQUEST,
        );
    }

    #[test]
    fn internal_server_error_returns_500() {
        assert_eq!(
            extract_status(AppError::InternalServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }

    #[test]
    fn sqlx_error_converts_to_app_error() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let app_err: AppError = sqlx_err.into();
        assert_eq!(extract_status(app_err), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
