use std::env;

use dotenv::dotenv;
use chrono::prelude::*;
use sqlx::FromRow;
use sqlx::postgres::{
    PgPoolOptions,
    types::PgRange,
}
#[derive(Debug, FromRow)]
struct Place {
    id: i32,
    name: String,
    capacity: i32,
    is_open: bool,
}

#[derive(Debug, FromRow)]
struct Reservation {
    id: i32,
    timespan: PgRange<DateTime<Utc>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //let db_url: String = "postgresql://postgres:postgres@localhost/coredb".to_owned();
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(2) //TODO: Read from .env
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    let select_query = sqlx::query_as::<_, Reservation>("SELECT id, timespan FROM reservation");
    let reservations: Vec<Reservation> = select_query.fetch_all(&pool).await?;

    dbg!("{:?}", reservations);

    Ok(())
}
