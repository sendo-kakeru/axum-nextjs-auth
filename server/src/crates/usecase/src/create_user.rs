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
