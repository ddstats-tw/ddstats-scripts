use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Updating players table");
    let players = Instant::now();

    sqlx::raw_sql(include_str!("../../sql/players.sql"))
        .execute(db)
        .await
        .unwrap();

    tracing::info!(
        "Finished generating players table in {:?}",
        players.elapsed()
    );

    Ok(())
}
