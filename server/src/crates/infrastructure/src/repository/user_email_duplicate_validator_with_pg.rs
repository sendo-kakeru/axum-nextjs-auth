use domain::{
    error::user_error::UserEmailDuplicateValidationError,
    interface::user_email_duplicate_validator_interface::UserEmailDuplicateValidatorInterface,
};

#[derive(Debug, Clone)]
pub struct UserEmailDuplicateValidatorWithPg {
    db: sqlx::PgPool,
}

impl UserEmailDuplicateValidatorWithPg {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db: db }
    }
}

#[async_trait::async_trait]
impl UserEmailDuplicateValidatorInterface for UserEmailDuplicateValidatorWithPg {
    async fn validate_user_email_duplicate(
        &self,
        email: &str,
    ) -> Result<(), UserEmailDuplicateValidationError> {
        let is_exist: bool =
            sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE email = $1)"#)
                .bind(email)
                .fetch_one(&self.db)
                .await?;

        if is_exist {
            return Err(UserEmailDuplicateValidationError::AlreadyExists);
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user_repository_with_pg::UserRepositoryWithPg;
    use domain::entity::user::User;
    use domain::interface::user_repository_interface::UserRepositoryInterface;

    async fn connect() -> Result<sqlx::PgPool, sqlx::Error> {
        dotenv::dotenv().ok();

        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
    }

    #[tokio::test]
    async fn test_validate_returns_ok_for_new_email() {
        let pool = connect().await.unwrap();
        let validator = UserEmailDuplicateValidatorWithPg::new(pool);
        let new_email = format!("unique+{}@example.com", uuid::Uuid::new_v4());
        let result = validator.validate_user_email_duplicate(&new_email).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_returns_err_for_existing_email() {
        let pool = connect().await.unwrap();
        let email = format!("test+{}@example.com", uuid::Uuid::new_v4());
        let user = User::new("Test User".into(), email.clone());
        let user_repo = UserRepositoryWithPg::new(pool.clone());
        user_repo.create(&user).await.expect("should insert user");
        let validator = UserEmailDuplicateValidatorWithPg::new(pool);
        let result = validator.validate_user_email_duplicate(&email).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "User email already exists"
        );
    }
}
