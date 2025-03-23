use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
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
    pub circle_id: String,
    pub owner_id: String,
}

impl std::convert::From<CreateUserOutput> for CreateUserResponseBody {
    fn from(create_user_output: CreateUserOutput) -> Self {
        CreateUserResponseBody {
            ..create_user_output.into()
        }
    }
}

pub(crate) async fn handle_create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> Result<Json<CreateUserResponseBody>, String> {
    let circle_circle_input = CreateUserInput::from(body);
    let mut usecase = CreateUserUsecase::new(state.user_repository);
    usecase
        .execute(circle_circle_input)
        .await
        .map(CreateUserResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
