use super::value_object::user_id::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        // pub fn new(name: String, email: String) -> Result<Self, anyhow::Error> {
        let id = UserId::new();
        User { id, name, email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_instantiate_test() {
        let user = User::new("Test User".into(), "test@example.com".into());
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
}
