#[derive(Debug, thiserror::Error)]
pub enum UserEmailDuplicateValidationError {
    #[error("User email already exists")]
    AlreadyExists,

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] sqlx::Error),
}
