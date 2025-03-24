use crate::model::user_model::UserModel;
use domain::entity::user::User;
use domain::interface::user_repository_interface::UserRepositoryInterface;

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
