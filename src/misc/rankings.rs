use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Running TRUNCATE on rankings, teamrankings");
    sqlx::query!("TRUNCATE rankings;").execute(db).await?;
    sqlx::query!("TRUNCATE teamrankings;").execute(db).await?;

    tracing::info!("Generating rankings");
    let rankings = Instant::now();
    sqlx::query_file!("sql/rankings.sql").execute(db).await?;
    tracing::info!("Finished generating rankings in {:?}", rankings.elapsed());

    let teamrankings = Instant::now();
    sqlx::query_file!("sql/teamrankings.sql")
        .execute(db)
        .await?;
    tracing::info!(
        "Finished generating teamrankings in {:?}",
        teamrankings.elapsed()
    );

    Ok(())
}
