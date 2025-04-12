use crate::usecase::find_user_by_id::FindUserByIdOutput;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FindUserByIdResponseBody {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl std::convert::From<FindUserByIdOutput> for FindUserByIdResponseBody {
    fn from(find_user_by_id_output: FindUserByIdOutput) -> Self {
        FindUserByIdResponseBody {
            id: find_user_by_id_output.id.0.to_string(),
            name: find_user_by_id_output.name,
            email: find_user_by_id_output.email,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::value_object::user_id::UserId;

    #[test]
    fn test_find_by_id_output() {
        let id = UserId::new();
        let output = FindUserByIdOutput {
            id: id.clone(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let response: FindUserByIdResponseBody = output.into();
        assert_eq!(response.id, id.0.to_string());
        assert_eq!(response.name, "Test User");
        assert_eq!(response.email, "test@example.com");
    }
}
