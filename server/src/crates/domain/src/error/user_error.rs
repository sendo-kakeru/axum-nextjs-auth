#[derive(Debug, thiserror::Error)]
pub enum UserEmailDuplicateValidationError {
    #[error("User email already exists error: {0}")]
    AlreadyExists(validator::ValidationError),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] sqlx::Error),
}
