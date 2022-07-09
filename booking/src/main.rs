use thiserror;
use std::env;
use sqlx::postgres::{
    PgPoolOptions,
};

mod model;
use model::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    let pool = PgPoolOptions::new()
        .max_connections(env::var("MAX_CONNECTIONS")?.parse::<u32>().unwrap())
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let select_query = sqlx::query_as::<_, Reservation>("SELECT id, timespan FROM reservation");
    let reservations: Vec<Reservation> = select_query.fetch_all(&pool).await?;

    let a = &reservations[0];
    let serialized = serde_json::to_string(&a).unwrap();
    println!("{}", serialized);

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}

