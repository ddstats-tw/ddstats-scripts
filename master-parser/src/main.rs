use chrono::{Duration, TimeZone, Utc};
use datatypes::DateEntry;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::panic;
use std::sync::mpsc::channel;
use std::thread;

pub mod database;
pub mod datatypes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to load dotenv");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    let (tx, rx) = channel::<DateEntry>();

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS playtime (
            date TEXT NOT NULL,
            location VARCHAR(8) NOT NULL DEFAULT 'unknown',
            gametype VARCHAR(32) NOT NULL,
            map VARCHAR(128) NOT NULL,
            name VARCHAR(15) NOT NULL,
            clan VARCHAR(15) NOT NULL,
            country INTEGER NOT NULL,
            skin_name VARCHAR(32),
            skin_color_body INTEGER,
            skin_color_feet INTEGER,
            afk BOOLEAN,
            team INTEGER,
            time INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS playtime_processed (
            date TEXT PRIMARY KEY
        )",
    )
    .execute(&pool)
    .await?;

    let start = Utc
        .with_ymd_and_hms(2021, 5, 18, 0, 0, 0)
        .single()
        .unwrap()
        .date_naive();
    let end = Utc::now().date_naive() - Duration::days(1);

    // Create a vector of all dates to process
    let mut dates: Vec<chrono::NaiveDate> = Vec::new();
    for dt in start.iter_days().take_while(|&dt| dt <= end) {
        let date_string = dt.format("%Y-%m-%d").to_string();
        let processed: Option<(String,)> =
            sqlx::query_as("SELECT date FROM playtime_processed WHERE date = $1")
                .bind(date_string)
                .fetch_optional(&pool)
                .await?;

        if processed.is_some() {
            println!("Already processed {}, skipping!", dt);
            continue;
        }
        dates.push(dt);
    }

    let writer_thread = tokio::spawn(async move {
        for mut date_entry in rx {
            match database::insert_snapshot(&mut date_entry, &pool).await {
                Ok(()) => continue,
                Err(e) => {
                    println!(
                        "An error occured while inserting {}, {:?}",
                        date_entry.date, e
                    );
                }
            };
        }
    });

    let num_workers: usize = match env::var("NUM_WORKERS") {
        Ok(num) => num.parse().expect("NUM_WORKERS is not a valid number"),
        Err(_) => thread::available_parallelism().unwrap().get() - 1,
    };

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(num_workers);
    for dt in dates {
        if handles.len() >= num_workers {
            let handle = handles.remove(0);
            handle.join().expect("Thread failed");
        }

        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                println!("{:?} - Processing {}", thread::current().id(), dt);
                let mut date_entry = DateEntry {
                    date: dt,
                    snapshot: HashMap::new(),
                };
                match database::process_day(&mut date_entry) {
                    Ok(_) => tx_clone.send(date_entry).unwrap(),
                    _ => {}
                }
            });
            match result {
                Err(e) => println!("An error occured while processing {} {:?}", dt, e),
                Ok(_) => {}
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed");
    }

    // Wait for all threads to complete
    drop(tx);
    writer_thread.await.unwrap();

    Ok(())
}
