use crate::{
    app::AppState,
    config::problem_type::{DUPLICATE, NOT_FOUND, VALIDATE},
};
use application::{
    request_response::{
        create_user_request::CreateUserRequestBody, create_user_response::CreateUserResponseBody,
        find_all_user_response::FindAllUserResponseBody,
    },
    usecase::{
        create_user::{CreateUserInput, CreateUserUsecase},
        find_all_user::FindAllUserUsecase,
    },
};
use axum::{
    extract::{Json, State},
    http::{self, StatusCode},
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
            .with_type(VALIDATE)
            .with_detail("One or more validation rules failed for the provided input")
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
            if let Some(UserEmailDuplicateValidationError::AlreadyExists) =
                e.downcast_ref::<UserEmailDuplicateValidationError>()
            {
                let problem = problemdetails::new(StatusCode::CONFLICT)
                    .with_title("Duplicate User Email")
                    .with_type(DUPLICATE)
                    .with_detail("This email address is already in use")
                    .with_instance("/users");

                Err(problem)
            } else {
                let problem = problemdetails::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .with_title("Internal Server Error")
                    .with_instance("/users");

                #[cfg(debug_assertions)]
                let problem = problem.with_detail(e.to_string());

                Err(problem)
            }
        }
    }
}

pub(crate) async fn handle_find_all_user(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, problemdetails::Problem> {
    let usecase = FindAllUserUsecase::new(state.user_repository);

    let output = usecase.execute().await.map_err(|e| {
        let problem = problemdetails::new(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Internal Server Error")
            .with_instance("/users");

        #[cfg(debug_assertions)]
        let problem = problem.with_detail(e.to_string());

        problem
    })?;
    let response_body = FindAllUserResponseBody::from(output);

    Ok((StatusCode::OK, Json(response_body)))
}

pub async fn handle_not_found(_req: http::Request<axum::body::Body>) -> impl IntoResponse {
    let problem = problemdetails::new(StatusCode::NOT_FOUND)
        .with_title("Not Found")
        .with_type(NOT_FOUND)
        .with_detail("The requested resource was not found.");
    problem
}
