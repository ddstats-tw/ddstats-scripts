use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Generating mostplayed leaderboard");
    let rankings = Instant::now();
    sqlx::query_file!("sql/most-played.sql").execute(db).await?;
    tracing::info!(
        "Finished generating mostplayed leaderboard in {:?}",
        rankings.elapsed()
    );

    Ok(())
}
