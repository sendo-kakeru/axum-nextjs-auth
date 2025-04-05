use crate::app::AppState;
use application::{
    request_response::{
        create_user_request::CreateUserRequestBody, create_user_response::CreateUserResponseBody,
    },
    usecase::create_user::{CreateUserInput, CreateUserUsecase},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::error::user_error::UserEmailDuplicateValidationError;
use validator::Validate;

pub(crate) async fn handle_create_user(
    State(state): State<AppState>,
    axum::Json(body): Json<CreateUserRequestBody>,
) -> Result<impl IntoResponse, problemdetails::Problem> {
    if let Err(validation_errors) = body.validate() {
        let mut problem = problemdetails::new(StatusCode::BAD_REQUEST)
            .with_title("Validation Error")
            .with_type("https://example.com/problems/validation")
            .with_detail("Value was not appropriate")
            .with_instance("/users");

        for (field, errors) in validation_errors.field_errors() {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();

            if !messages.is_empty() {
                problem = problem.with_value(&field, messages);
            }
        }

        return Err(problem);
    }

    let create_user_input = CreateUserInput::from(body);
    let mut usecase =
        CreateUserUsecase::new(state.user_repository, state.user_email_duplicate_validator);

    match usecase.execute(create_user_input).await {
        Ok(user) => {
            let response_body = CreateUserResponseBody::from(user);
            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(e) => {
            if let Some(UserEmailDuplicateValidationError::AlreadyExists(validation_error)) =
                e.downcast_ref::<UserEmailDuplicateValidationError>()
            {
                let problem = problemdetails::new(StatusCode::BAD_REQUEST)
                    .with_title("Validation Error")
                    .with_type("https://example.com/problems/validation")
                    .with_detail("Value was not appropriate")
                    .with_instance("/users")
                    .with_value(
                        "email",
                        validation_error
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_default(),
                    );

                Err(problem)
            } else {
                let problem = problemdetails::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .with_title("Internal Server Error")
                    .with_instance("/users")
                    .with_detail(e.to_string());
                Err(problem)
            }
        }
    }
}
