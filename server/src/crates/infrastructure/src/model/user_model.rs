use domain::entity::{user::User, value_object::user_id::UserId};
use uuid::Uuid;

pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl TryFrom<UserModel> for User {
  type Error = anyhow::Error;

  fn try_from(model: UserModel) -> Result<Self, Self::Error> {
      Ok(User {
          id: UserId::from(model.id),
          name: model.name,
          email: model.email,
      })
  }
}

impl From<User> for UserModel {
  fn from(user: User) -> Self {
      UserModel {
          id: user.id.into(),
          name: user.name,
          email: user.email,
      }
  }
}