use domain::{entity::user::User, interface::user_repository_interface::UserRepositoryInterface};

#[derive(Debug, Clone, PartialEq)]
pub struct FindAllUserOutput(pub Vec<User>);
pub struct FindAllUserUsecase<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> FindAllUserUsecase<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        FindAllUserUsecase { user_repository }
    }

    pub async fn execute(&self) -> anyhow::Result<FindAllUserOutput> {
        let users = self.user_repository.find_all().await?;
        anyhow::Ok(FindAllUserOutput(users))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use domain::{
        entity::user::User, interface::user_repository_interface::MockUserRepositoryInterface,
    };

    #[tokio::test]
    async fn test_find_all_user_usecase_successful() -> anyhow::Result<()> {
        let mut mocked_user_repository = MockUserRepositoryInterface::new();

        let user1 = Arc::new(User::new(
            "Test User1".into(),
            format!("test+{}@example.com", uuid::Uuid::new_v4()),
        ));
        let user2 = Arc::new(User::new(
            "Test User2".into(),
            format!("test+{}@example.com", uuid::Uuid::new_v4()),
        ));

        let u1 = user1.clone();
        let u2 = user2.clone();

        mocked_user_repository
            .expect_find_all()
            .returning(move || Ok(vec![(*u1).clone(), (*u2).clone()]));

        let usecase = FindAllUserUsecase::new(mocked_user_repository);
        let output = usecase.execute().await.unwrap();

        assert_eq!(
            output,
            FindAllUserOutput(vec![(*user1).clone(), (*user2).clone()])
        );

        Ok(())
    }
}
