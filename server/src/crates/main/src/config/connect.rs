pub async fn connect() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

// #[cfg(test)]
// pub async fn test_connect() -> Result<sqlx::PgPool, sqlx::Error> {
//     dotenv::dotenv().ok();

//     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = sqlx::postgres::PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await?;

//     Ok(pool)
// }
