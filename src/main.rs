use std::{borrow::Borrow, sync::Arc, time::Instant};

use clap::{Parser, Subcommand};
use database::init_db;

mod database;
#[path = "./database-sync/mod.rs"]
mod database_sync;
#[path = "./historical-rankedpoints/mod.rs"]
mod historical_rankedpoints;
#[path = "./master-parser/mod.rs"]
mod master_parser;
mod misc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Sync,
    #[clap(name = "master-parser")]
    MasterParser,
    Rankings,
    #[clap(name = "most-played")]
    MostPlayed,
    #[clap(name = "map-stats")]
    MapStats,
    #[clap(name = "sync-database")]
    SyncDatabase,
    Players,
    #[clap(name = "ranked-points")]
    RankedPoints,
    #[clap(name = "playtime-maps")]
    PlaytimeMaps,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let db = Arc::new(init_db().await);

    tracing::info!("Creating tables");
    let _ = sqlx::raw_sql(include_str!("../sql/create-tables.sql"))
        .execute(db.borrow())
        .await;

    match &cli.action {
        Actions::Sync => {
            tracing::info!("Starting complete sync of database");
            let sync = Instant::now();
            database_sync::main(db.borrow()).await;
            master_parser::main(db.clone()).await.ok();
            misc::most_played::main(db.borrow()).await.ok();
            //misc::playtime_maps::main(db.borrow()).await.ok();
            historical_rankedpoints::main(db.borrow()).await;
            misc::map_stats::main(db.borrow()).await.ok();
            misc::rankings::main(db.borrow()).await.ok();
            misc::players::main(db.borrow()).await.ok();
            tracing::info!("Database sync took {:?} to complete", sync.elapsed());
        }
        Actions::MasterParser => {
            master_parser::main(db.clone()).await.ok();
        }
        Actions::Rankings => {
            misc::rankings::main(db.borrow()).await.ok();
        }
        Actions::MostPlayed => {
            misc::most_played::main(db.borrow()).await.ok();
        }
        Actions::MapStats => {
            misc::map_stats::main(db.borrow()).await.ok();
        }
        Actions::Players => {
            misc::players::main(db.borrow()).await.ok();
        }
        Actions::SyncDatabase => {
            database_sync::main(db.borrow()).await;
        }
        Actions::RankedPoints => {
            historical_rankedpoints::main(db.borrow()).await;
        }
        Actions::PlaytimeMaps => {
            misc::playtime_maps::main(db.borrow()).await.ok();
        }
    }

    tracing::info!("Creating indexes");
    let _ = sqlx::raw_sql(include_str!("../sql/create-indexes.sql"))
        .execute(db.borrow())
        .await;
}
