use domain::{
    entity::{user::User, value_object::user_id::UserId},
    interface::user_repository_interface::UserRepositoryInterface,
};

pub type FindUserByIdInput = UserId;

pub type FindUserByIdOutput = User;
pub struct FindUserByIdUsecase<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> FindUserByIdUsecase<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        FindUserByIdUsecase { user_repository }
    }

    pub async fn execute(
        &self,
        find_user_by_id_input: FindUserByIdInput,
    ) -> anyhow::Result<FindUserByIdOutput> {
        let user = self
            .user_repository
            .find_by_id(&find_user_by_id_input)
            .await?;
        anyhow::Ok(user)
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Ok;
    use domain::interface::user_repository_interface::MockUserRepositoryInterface;

    use super::*;

    #[tokio::test]
    async fn test_find_by_id_usecase_successful() -> anyhow::Result<()> {
        let mut mocked_user_repository = MockUserRepositoryInterface::new();
        let user = User::new("Test User".into(), format!("test@example.com"));
        let user_id = user.id.clone();

        mocked_user_repository.expect_find_by_id().returning({
            let user = user.clone();
            move |_user_id| Ok(user.clone())
        });
        let usecase = FindUserByIdUsecase::new(mocked_user_repository);
        let result = usecase.execute(user_id).await?;
        assert_eq!(result, user);
        anyhow::Ok(())
    }
}
