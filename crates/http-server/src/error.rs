use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;

#[derive(ErrorResponse, thiserror::Error)]
pub enum HttpException {
    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    PathRejectionError(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    FormRejectionError(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    QueryRejectionError(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    BodyRejectionError(#[from] axum::extract::rejection::JsonRejection),

    #[error("{0:#?}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequest(String),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(String),

    #[error("{0:#?}")]
    InternalError(String),

    #[error(transparent)]
    MissingEnv(#[from] std::env::VarError),

    #[error("Database error: {0:#?}")]
    DatabaseError(#[from] database::sea_orm::error::DbErr),
}
