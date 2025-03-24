use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::entity::value_object::user_id::UserId;
use serde::Deserialize;
use std::env;
use usecase::create_user::{CreateUserInput, CreateUserOutput, CreateUserUsecase};

use crate::app::AppState;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserRequestBody> for CreateUserInput {
    fn from(CreateUserRequestBody { name, email }: CreateUserRequestBody) -> Self {
        CreateUserInput::new(name, email)
    }
}

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

pub(crate) async fn handle_create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> Result<Json<CreateUserResponseBody>, String> {
    let create_user_input = CreateUserInput::from(body);
    let mut usecase = CreateUserUsecase::new(state.user_repository);
    usecase
        .execute(create_user_input)
        .await
        .map(CreateUserResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
