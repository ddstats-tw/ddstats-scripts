use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Running TRUNCATE on players");
    sqlx::query!("TRUNCATE players;").execute(db).await?;

    tracing::info!("Generating players table");
    let players = Instant::now();
    sqlx::query_file!("sql/players.sql").execute(db).await?;
    tracing::info!(
        "Finished generating players table in {:?}",
        players.elapsed()
    );

    Ok(())
}
