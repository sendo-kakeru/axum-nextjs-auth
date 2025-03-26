use crate::model::user_model::UserModel;
use domain::entity::user::User;
use domain::interface::user_repository_interface::UserRepositoryInterface;
// use main:

#[derive(Debug, Clone)]
pub struct UserRepositoryWithPg {
    db: sqlx::PgPool,
}

impl UserRepositoryWithPg {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db: db }
    }
}

#[async_trait::async_trait]
impl UserRepositoryInterface for UserRepositoryWithPg {
    async fn create(&self, user: &User) -> Result<User, anyhow::Error> {
        tracing::info!("create_user: {:?}", user);
        let user_model = UserModel::try_from(user.clone())?;
        let row = sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO "user" (id, name, email)
            VALUES ($1, $2, $3)
            RETURNING id, name, email
            "#,
            user_model.id,
            user_model.name,
            user_model.email
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert user: {:?}", e);
            anyhow::Error::msg("Failed to insert user")
        })?;

        Ok(User::try_from(row)?)
    }
}

#[cfg(test)]
mod tests {
    use domain::{
        entity::user::User, interface::user_repository_interface::UserRepositoryInterface,
    };

    use super::UserRepositoryWithPg;

    async fn connect() -> Result<sqlx::PgPool, sqlx::Error> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn test() {
        let pool = connect().await.expect("database should connect");
        let user_repository = UserRepositoryWithPg::new(pool.clone());
        let user = User::new("Test User".into(), "test@example.com".into());
        let created_user = user_repository.create(&user).await.unwrap();

        assert_eq!(created_user.name, user.name);
        assert_eq!(created_user.email, user.email);
    }
}
