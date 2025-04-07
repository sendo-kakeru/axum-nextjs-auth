use crate::model::user_model::UserModel;
use domain::entity::user::User;
use domain::interface::user_repository_interface::UserRepositoryInterface;

#[derive(Debug, Clone)]
pub struct UserRepositoryWithPg {
    db: sqlx::PgPool,
}

impl UserRepositoryWithPg {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
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

    async fn find_all(&self) -> Result<Vec<User>, anyhow::Error> {
        let rows = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, name, email FROM "user"
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch users: {:?}", e);
            anyhow::Error::msg("Failed to fetch users")
        })?;

        rows.into_iter()
            .map(User::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                eprintln!("Failed to convert UserModel to User: {:?}", e);
                anyhow::Error::msg("Data conversion failed")
            })
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

        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn test_create_user_successfully() {
        let email = format!("test+{}@example.com", uuid::Uuid::new_v4());
        let pool = connect().await.expect("database should connect");
        let user_repository = UserRepositoryWithPg::new(pool.clone());
        let user = User::new("Test User".into(), email.into());
        let created_user = user_repository
            .create(&user)
            .await
            .expect("should successfully create user");

        assert_eq!(created_user.name, user.name);
        assert_eq!(created_user.email, user.email);
    }

    #[tokio::test]
    async fn test_find_all_users_successfully() {
        let pool = connect().await.expect("database should connect");
        let user_repository = UserRepositoryWithPg::new(pool.clone());

        let email1 = format!("user1+{}@example.com", uuid::Uuid::new_v4());
        let user1 = User::new("User One".into(), email1.clone());
        user_repository
            .create(&user1)
            .await
            .expect("should create user 1");

        let email2 = format!("user2+{}@example.com", uuid::Uuid::new_v4());
        let user2 = User::new("User Two".into(), email2.clone());
        user_repository
            .create(&user2)
            .await
            .expect("should create user 2");

        let users = user_repository
            .find_all()
            .await
            .expect("should fetch all users");

        assert!(
            users.iter().any(|u| u.email == email1),
            "User 1 should exist"
        );
        assert!(
            users.iter().any(|u| u.email == email2),
            "User 2 should exist"
        );
    }
}
