use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Generating mapstats table");
    let rankings = Instant::now();
    sqlx::query_file!("sql/mapstats.sql").execute(db).await?;
    tracing::info!(
        "Finished generating mapstats table in {:?}",
        rankings.elapsed()
    );

    Ok(())
}
