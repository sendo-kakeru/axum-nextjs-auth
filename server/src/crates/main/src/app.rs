use axum::{
    Router,
    routing::{get, post, put},
};
use infrastructure::repository::user_repository_with_pg::UserRepositoryWithPg;

use crate::{config::connect, handler::handle_create_user};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) user_repository: UserRepositoryWithPg,
}

fn router() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Home" })).route(
        "/api/users",
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
