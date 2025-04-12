use crate::usecase::create_user::CreateUserOutput;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateUserResponseBody {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserOutput> for CreateUserResponseBody {
    fn from(create_user_output: CreateUserOutput) -> Self {
        CreateUserResponseBody {
            id: create_user_output.id.0.to_string(),
            name: create_user_output.name,
            email: create_user_output.email,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::value_object::user_id::UserId;

    #[test]
    fn test_create_user_output() {
        let id = UserId::new();
        let output = CreateUserOutput {
            id: id.clone(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let response: CreateUserResponseBody = output.into();

        assert_eq!(response.id, id.0.to_string());
        assert_eq!(response.name, "Test User");
        assert_eq!(response.email, "test@example.com");
    }

    #[test]
    fn test_serialize_to_json() {
        let response = CreateUserResponseBody {
            id: "79ca0feb-84f2-4e75-ae07-fc0dd877f9ce".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        let expected = r#"{"id":"79ca0feb-84f2-4e75-ae07-fc0dd877f9ce","name":"Test User","email":"test@example.com"}"#;
        assert_eq!(json, expected);
    }
}
