use core::ops::Bound::{Excluded, Included, Unbounded};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::types::PgRange;


#[derive(FromRow, Serialize,Debug)]
pub struct Reservation {
    pub id: i32,
    #[serde(serialize_with = "serialize_range", flatten)]
    pub timespan: PgRange<DateTime<Utc>>,
}

fn serialize_range<S, T>(range: &PgRange<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: Serialize,
{
    let PgRange { start, end } = range;
    std::ops::Range { start, end }.serialize(serializer)
}

