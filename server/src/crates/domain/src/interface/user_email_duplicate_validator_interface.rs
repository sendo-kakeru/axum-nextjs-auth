use crate::error::user_error::UserEmailDuplicateValidationError;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserEmailDuplicateValidatorInterface {
    async fn validate_user_email_duplicate(&self, email: &str) -> Result<(), UserEmailDuplicateValidationError>;
}
