use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Generating rankings");
    let rankings = Instant::now();
    sqlx::raw_sql(include_str!("../../sql/rankings.sql"))
        .execute(db)
        .await
        .unwrap();
    tracing::info!("Finished generating rankings in {:?}", rankings.elapsed());

    let teamrankings = Instant::now();
    sqlx::raw_sql(include_str!("../../sql/teamrankings.sql"))
        .execute(db)
        .await
        .unwrap();
    tracing::info!(
        "Finished generating teamrankings in {:?}",
        teamrankings.elapsed()
    );

    Ok(())
}
