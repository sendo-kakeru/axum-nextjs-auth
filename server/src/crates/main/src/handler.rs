use crate::app::AppState;
use application::{
    request_response::{
        create_user_request::CreateUserRequestBody, create_user_response::CreateUserResponseBody,
    },
    usecase::create_user::{CreateUserInput, CreateUserUsecase},
};
use axum::extract::{Json, State};
use axum_valid::Valid;

pub(crate) async fn handle_create_user(
    State(state): State<AppState>,
    Valid(json): Valid<Json<CreateUserRequestBody>>,
) -> Result<Json<CreateUserResponseBody>, String> {
    let body = json.0;
    let create_user_input = CreateUserInput::from(body);
    let mut usecase =
        CreateUserUsecase::new(state.user_repository, state.user_email_duplicate_validator);
    usecase
        .execute(create_user_input)
        .await
        .map(CreateUserResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
