use domain::entity::{user::User, value_object::user_id::UserId};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_model_to_user_conversion_works() {
        let uuid = Uuid::new_v4();
        let model = UserModel {
            id: uuid,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let user = User::try_from(model).unwrap();
        assert_eq!(Uuid::from(user.id.clone()), uuid);
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn user_to_user_model_conversion_works() {
        let uuid = Uuid::new_v4();
        let user = User {
            id: UserId::from(uuid),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let model: UserModel = user.into();
        assert_eq!(model.id, uuid);
        assert_eq!(model.name, "Test User");
        assert_eq!(model.email, "test@example.com");
    }
}
