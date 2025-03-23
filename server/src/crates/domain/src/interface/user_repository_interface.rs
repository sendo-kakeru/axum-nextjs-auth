use anyhow::Error;

use crate::entity::user::User;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepositoryInterface {
    // async fn find_all(&self) -> Result<Vec<Circle>, Error>;
    // async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn create(&self, user: &User) -> Result<(), Error>;
    // async fn update(&self, circle: &Circle) -> Result<Circle, Error>;
    // async fn delete(&self, circle: &Circle) -> Result<(), Error>;
}
