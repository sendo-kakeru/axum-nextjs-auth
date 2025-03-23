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
