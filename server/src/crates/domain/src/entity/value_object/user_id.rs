use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]

pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let user_id = UserId::from(uuid);
        user_id.into()
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
