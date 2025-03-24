use crate::entity::user::User;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepositoryInterface {
    async fn create(&self, user: &User) -> Result<User, anyhow::Error>;
}
