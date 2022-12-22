use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Intended to be returned in a `Result` from APIs.
/// Both unexpected and expected errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// `401`
    #[error("Authentication required")]
    Unauthorized,

    /// `403`
    #[error("Not authorized")]
    Forbidden,

    /// `404`
    #[error("Not found")]
    NotFound,

    /// todo: add parsing for unprocessable entity, matching FastAPI/pydantic
    /// `422`, similar to FastAPI.
    #[error("Bad input")]
    UnprocessableEntity,

    /// Return 500 on an `anyhow:Error`
    /// `500`
    #[error("Internal Server Error")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    // todo: parse unprocessable entity
    //  pub fn unprocessable_entity...

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    detail: String
}



impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            // todo add Self::UnprocessableEntity and parse it
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    Json(ErrorResponse {detail: self.to_string()}),
                ).into_response();
            }
            // Make sure uncaught errors/anyhow errors are not displayed out, but log them.
            Self::Anyhow(ref e) => {
                // todo: unsure why this don't show in terminal, investigate
                tracing::error!("Uncaught error: {:?}", e);
            }
            // All other errors, just throw error
            _ => (),
        }
        let error_response = ErrorResponse {detail: self.to_string()};
        (self.status_code(), Json(error_response)).into_response()
    }
}
