use crate::app::AppState;
use axum::extract::{Json, State};
use usecase::create_user::{CreateUserInput, CreateUserOutput, CreateUserUsecase};
use axum_valid::Valid;
use validator::Validate;

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
    Valid(json): Valid<Json<CreateUserRequestBody>>,
) -> Result<Json<CreateUserResponseBody>, String> {
    let body = json.0;
    let create_user_input = CreateUserInput::from(body);
    let mut usecase = CreateUserUsecase::new(state.user_repository);
    usecase
        .execute(create_user_input)
        .await
        .map(CreateUserResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
