use domain::entity::user::User;
use serde::Serialize;

use crate::usecase::find_all_user::FindAllUserOutput;

#[derive(Debug, Serialize)]
pub struct FindAllUserResponseBodyItem {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for FindAllUserResponseBodyItem {
    fn from(user: User) -> Self {
        FindAllUserResponseBodyItem {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FindAllUserResponseBody(pub Vec<FindAllUserResponseBodyItem>);

impl From<FindAllUserOutput> for FindAllUserResponseBody {
    fn from(output: FindAllUserOutput) -> Self {
        let items = output
            .0
            .into_iter()
            .map(FindAllUserResponseBodyItem::from)
            .collect();
        FindAllUserResponseBody(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecase::find_all_user::FindAllUserOutput;
    use domain::entity::user::User;
    use domain::entity::value_object::user_id::UserId;

    #[test]
    fn test_find_all_user_output() {
        let user1 = User {
            id: UserId::new(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        let user2 = User {
            id: UserId::new(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        };

        let output = FindAllUserOutput(vec![user1.clone(), user2.clone()]);
        let response: FindAllUserResponseBody = output.into();

        assert_eq!(response.0.len(), 2);

        assert_eq!(response.0[0].id, user1.id.to_string());
        assert_eq!(response.0[0].name, user1.name);
        assert_eq!(response.0[0].email, user1.email);

        assert_eq!(response.0[1].id, user2.id.to_string());
        assert_eq!(response.0[1].name, user2.name);
        assert_eq!(response.0[1].email, user2.email);
    }

    #[test]
    fn test_serialize_find_all_user_response_body_to_json() {
        let response = FindAllUserResponseBody(vec![
            FindAllUserResponseBodyItem {
                id: "id-1".to_string(),
                name: "Test User 1".to_string(),
                email: "user1@example.com".to_string(),
            },
            FindAllUserResponseBodyItem {
                id: "id-2".to_string(),
                name: "Test User 2".to_string(),
                email: "user2@example.com".to_string(),
            },
        ]);

        let json = serde_json::to_string(&response).unwrap();
        let expected = r#"[{"id":"id-1","name":"Test User 1","email":"user1@example.com"},{"id":"id-2","name":"Test User 2","email":"user2@example.com"}]"#;

        assert_eq!(json, expected);
    }
}
