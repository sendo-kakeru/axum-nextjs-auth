use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]

pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        UserId(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}
