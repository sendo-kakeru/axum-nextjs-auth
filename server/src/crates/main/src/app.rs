use crate::{config::connect, handler::handle_create_user};
use axum::{Router, routing::get};
use infrastructure::repository::user_repository_with_pg::UserRepositoryWithPg;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) user_repository: UserRepositoryWithPg,
}

fn router() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Home" })).route(
        "/users",
        get(|| async { "Home" }).post(handle_create_user),
    )
}

pub async fn run() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let pool = connect::connect().await.expect("database should connect");
    let state = AppState {
        user_repository: UserRepositoryWithPg::new(pool.clone()),
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
    use anyhow::Ok;
    use axum::http::{StatusCode, header::CONTENT_TYPE};
    use tower::ServiceExt;

    use crate::handler::{CreateUserRequestBody, CreateUserResponseBody};

    use super::*;

    async fn connect() -> anyhow::Result<sqlx::PgPool, anyhow::Error> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_user() -> anyhow::Result<()> {
        let pool = connect().await.expect("database should connect");
        let state = AppState {
            user_repository: UserRepositoryWithPg::new(pool.clone()),
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
                            email: "test@example.com".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = serde_json::from_slice::<'_, CreateUserResponseBody>(
            &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
        )?;
        assert_eq!(response_body.name, "Test User");
        assert_eq!(response_body.email, "test@example.com");
        assert!(!response_body.id.is_empty());
        Ok(())

        // @todo 取得ができたら検証テスト追加
    }
}
