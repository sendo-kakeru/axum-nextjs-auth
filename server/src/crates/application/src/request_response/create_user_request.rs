use validator::Validate;

use crate::usecase::create_user::CreateUserInput;

#[derive(Debug, serde::Deserialize, serde::Serialize, Validate)]
pub struct CreateUserRequestBody {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl std::convert::From<CreateUserRequestBody> for CreateUserInput {
    fn from(CreateUserRequestBody { name, email }: CreateUserRequestBody) -> Self {
        CreateUserInput::new(name, email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_request_passes_validation() {
        let req = CreateUserRequestBody {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_too_short_name_fails_validation() {
        let req = CreateUserRequestBody {
            name: "A".to_string(),
            email: "test@example.com".to_string(),
        };

        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("name"));
    }

    #[test]
    fn test_max_name_length_is_valid() {
        let req = CreateUserRequestBody {
            name: "A".repeat(50),
            email: "test@example.com".to_string(),
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_name_exceeding_max_length_fails_validation() {
        let req = CreateUserRequestBody {
            name: "A".repeat(51),
            email: "test@example.com".to_string(),
        };

        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("name"));
    }

    #[test]
    fn test_invalid_email_fails_validation() {
        let req = CreateUserRequestBody {
            name: "Test User".to_string(),
            email: "not-an-email".to_string(),
        };

        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn test_from_request_body_to_input() {
        let req = CreateUserRequestBody {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let input: CreateUserInput = req.into();

        assert_eq!(input.name, "Test User");
        assert_eq!(input.email, "test@example.com");
    }
}
