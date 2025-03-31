use anyhow::Ok;
use domain::interface::user_email_duplicate_validator_interface::UserEmailDuplicateValidatorInterface;

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
    async fn validate_user_email_duplicate(&self, email: &str) -> Result<(), anyhow::Error> {
        let is_exist: bool =
            sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE email = $1)"#)
                .bind(email)
                .fetch_one(&self.db)
                .await?;

        if is_exist {
            return Err(anyhow::anyhow!("email is already registered"));
        }

        Ok(())
    }
}

// @todo テスト用DBを作成したらテスト追加
// #[cfg(test)]
// mod tests {
//     use domain::{
//         entity::user::User, interface::user_repository_interface::UserRepositoryInterface,
//     };

//     use super::UserRepositoryWithPg;

//     async fn connect() -> Result<sqlx::PgPool, sqlx::Error> {
//         dotenv::dotenv().ok();

//         let database_url = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&database_url)
//             .await?;

//         Ok(pool)
//     }

//     #[tokio::test]
//     async fn test_create_user_successfully() {
//         let pool = connect().await.expect("database should connect");
//         let user_repository = UserRepositoryWithPg::new(pool.clone());
//         let user = User::new("Test User".into(), "test@example.com".into());
//         let created_user = user_repository
//             .create(&user)
//             .await
//             .expect("should successfully create user");

//         assert_eq!(created_user.name, user.name);
//         assert_eq!(created_user.email, user.email);
//     }
// }
