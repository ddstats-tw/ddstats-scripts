use sqlx::{Pool, Postgres};
use std::{error::Error, time::Instant};

pub async fn main(db: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    tracing::info!("Updating playtime_maps table");
    let playtime_maps = Instant::now();
    sqlx::query_file!("sql/playtime-maps.sql").execute(db).await?;
    tracing::info!("Finished updating playtime_maps table in {:?}", playtime_maps.elapsed());

    sqlx::query_file!("sql/playtime-maps-processed.sql").execute(db).await?;

    Ok(())
}
