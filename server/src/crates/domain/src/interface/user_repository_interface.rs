use crate::entity::{user::User, value_object::user_id::UserId};

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepositoryInterface {
    async fn create(&self, user: &User) -> Result<User, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<User>, anyhow::Error>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<User, anyhow::Error>;
}
