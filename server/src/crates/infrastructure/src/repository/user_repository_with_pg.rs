use domain::interface::user_repository_interface::UserRepositoryInterface;
use domain::entity::user::User;
use crate::model::user_model::UserModel;

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
    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        tracing::info!("create_user: {:?}", user);
        let user_model = UserModel::try_from(user.clone())?;
        sqlx::query(
            r#"
        insert into
            user (id, name, email)
        values
            (?, ?, ?)
      "#,
        )
        .bind(user_model.id)
        .bind(user_model.name)
        .bind(user_model.email)
        .execute(&self.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert user: {:?}", e);
            anyhow::Error::msg("Failed to insert user")
        })?;

        Ok(())
    }
}
