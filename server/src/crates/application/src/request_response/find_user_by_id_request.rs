use domain::entity::value_object::user_id::UserId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindUserByIdRequestParam {
    pub id: UserId,
}
