use crate::{
    config::connect,
    handler::{handle_create_user, handle_not_found},
};
use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use infrastructure::repository::{
    user_email_duplicate_validator_with_pg::UserEmailDuplicateValidatorWithPg,
    user_repository_with_pg::UserRepositoryWithPg,
};
use tower::ServiceBuilder;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) user_repository: UserRepositoryWithPg,
    pub(crate) user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/users", get(|| async { "Home" }).post(handle_create_user))
        .layer(ServiceBuilder::new().layer(axum::middleware::map_response(
            |response: Response| async {
                match response.status() {
                    StatusCode::UNPROCESSABLE_ENTITY => {
                        return (
                            StatusCode::UNPROCESSABLE_ENTITY,
                            problemdetails::new(StatusCode::UNPROCESSABLE_ENTITY)
                                .with_title("Invalid JSON")
                                .with_type("https://example.com/problems/invalid-json")
                                .with_detail("Required fields are missing or invalid")
                                .with_instance("/users"),
                        )
                            .into_response();
                    }
                    StatusCode::METHOD_NOT_ALLOWED => {
                        return (problemdetails::new(StatusCode::METHOD_NOT_ALLOWED)
                            .with_title("Method Not Allowed")
                            .with_type("https://example.com/problems/method-not-allowed"))
                        .into_response();
                    }
                    StatusCode::BAD_REQUEST => {
                        return (
                            StatusCode::BAD_REQUEST,
                            problemdetails::new(StatusCode::BAD_REQUEST)
                                .with_title("Bad Request")
                                .with_type("https://example.com/problems/bad-request")
                                .with_detail("Malformed query or path parameter"),
                        )
                            .into_response();
                    }
                    StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                        return (
                            StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            problemdetails::new(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                                .with_title("Unsupported Media Type")
                                .with_type("https://example.com/problems/unsupported-media-type")
                                .with_detail("Content-Type must be application/json"),
                        )
                            .into_response();
                    }
                    status if !status.is_success() => {
                        return problemdetails::new(StatusCode::INTERNAL_SERVER_ERROR)
                            .with_title("Unexpected Error")
                            .with_type("https://example.com/problems/internal-server-error")
                            .with_detail(format!("Unhandled status: {}", status))
                            .into_response();
                    }
                    _ => response,
                }
            },
        )))
        .layer(
            problemdetails::axum::PanicHandlerBuilder::new()
                .with_problem(
                    problemdetails::new(StatusCode::INTERNAL_SERVER_ERROR)
                        .with_title("Internal Server Error"),
                )
                .build(),
        )
        .fallback(handle_not_found)
}

pub async fn run() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let pool = connect::connect().await.expect("database should connect");
    let state = AppState {
        user_repository: UserRepositoryWithPg::new(pool.clone()),
        user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
    };

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use application::request_response::{
        create_user_request::CreateUserRequestBody, create_user_response::CreateUserResponseBody,
    };
    use axum::http::{StatusCode, header::CONTENT_TYPE};
    use tower::ServiceExt;

    use super::*;

    async fn connect() -> anyhow::Result<sqlx::PgPool, anyhow::Error> {
        dotenv::dotenv().ok();

        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let email = format!("test+{}@example.com", uuid::Uuid::new_v4());
        let pool = connect().await.expect("database should connect");
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };

        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateUserRequestBody {
                            name: "Test User".to_string(),
                            email: email.clone().to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::CREATED);
        let response_body = serde_json::from_slice::<'_, CreateUserResponseBody>(
            &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
        )?;
        assert_eq!(response_body.name, "Test User");
        assert_eq!(response_body.email, email.clone());
        assert!(!response_body.id.is_empty());
        Ok(())

        // @todo 取得ができたら検証テスト追加
    }

    #[tokio::test]
    async fn test_create_user_422() {
        let pool = connect().await.unwrap();
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };
        let app = router().with_state(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(r#"{}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let problem: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(problem["title"], "Invalid JSON");
        assert_eq!(problem["type"], "https://example.com/problems/invalid-json");
    }

    #[tokio::test]
    async fn test_create_user_405() {
        let pool = connect().await.unwrap();
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };
        let app = router().with_state(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PUT")
                    .uri("/users")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(r#"{}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let problem: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(problem["title"], "Method Not Allowed");
        assert_eq!(
            problem["type"],
            "https://example.com/problems/method-not-allowed"
        );
    }

    #[tokio::test]
    async fn test_create_user_400() -> anyhow::Result<()> {
        let pool = connect().await.expect("database should connect");
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };

        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateUserRequestBody {
                            name: "Test User".to_string(),
                            email: "Bad email".to_string(),
                        },
                    )?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let problem: serde_json::Value = serde_json::from_slice(&body)?;

        assert_eq!(problem["title"], "Bad Request");
        assert_eq!(problem["type"], "https://example.com/problems/bad-request");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_415() {
        let pool = connect().await.unwrap();
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };
        let app = router().with_state(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    // .method("POST")
                    // .uri("/users")
                    // .header(CONTENT_TYPE, "application/json")
                    // .body(axum::body::Body::from(r#"{}"#))
                    // .unwrap(),
                    .method("POST")
                    .uri("/users")
                    .header(CONTENT_TYPE, "text/plain")
                    .body(axum::body::Body::from(r#"{}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let problem: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(problem["title"], "Unsupported Media Type");
        assert_eq!(
            problem["type"],
            "https://example.com/problems/unsupported-media-type"
        );
    }
    #[tokio::test]
    async fn test_create_user_with_invalid_json() {
        let pool = connect().await.unwrap();
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
            user_email_duplicate_validator: UserEmailDuplicateValidatorWithPg::new(pool.clone()),
        };
        let app = router().with_state(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(r#"{}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let problem: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(problem["title"], "Invalid JSON");
        assert_eq!(problem["type"], "https://example.com/problems/invalid-json");
    }
}
