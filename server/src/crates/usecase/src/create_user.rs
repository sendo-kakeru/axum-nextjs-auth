use domain::{entity::user::User, interface::user_repository_interface::UserRepositoryInterface};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

impl CreateUserInput {
    pub fn new(name: String, email: String) -> Self {
        CreateUserInput { name, email }
    }
}

pub type CreateUserOutput = User;

pub struct CreateUserUsecase<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> CreateUserUsecase<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        CreateUserUsecase { user_repository }
    }

    pub async fn execute(
        &mut self,
        create_user_input: CreateUserInput,
    ) -> anyhow::Result<CreateUserOutput> {
        let user = User::new(create_user_input.name, create_user_input.email);
        self.user_repository
            .create(&user)
            .await
            .map(|_| CreateUserOutput { ..user })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{
        entity::value_object::user_id::UserId,
        interface::user_repository_interface::MockUserRepositoryInterface,
    };

    #[tokio::test]
    async fn test_create_user_usecase_successful() -> anyhow::Result<()> {
        let mut mocked_user_repository = MockUserRepositoryInterface::new();

        let input = CreateUserInput::new("Test User".into(), "test@example.com".into());
        let expected_user = User {
            id: UserId::new(),
            name: input.name.clone(),
            email: input.email.clone(),
        };

        // クロージャ用に clone しておく（ムーブしても OK）
        let expected_name_for_match = expected_user.name.clone();
        let expected_email_for_match = expected_user.email.clone();

        let expected_name = expected_user.name.clone();
        let expected_email = expected_user.email.clone();

        mocked_user_repository
            .expect_create()
            .withf(move |user| {
                user.name == expected_name_for_match && user.email == expected_email_for_match
            })
            .returning(move |_user| Ok(expected_user.clone()));

        let mut usecase = CreateUserUsecase::new(mocked_user_repository);
        let result = usecase.execute(input).await.unwrap();

        assert_eq!(&result.name, &expected_name);
        assert_eq!(&result.email, &expected_email);

        anyhow::Ok(())
    }
}
