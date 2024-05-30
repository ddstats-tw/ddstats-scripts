use chrono::{Duration, NaiveDate, Utc};
use datatypes::DateEntry;
use sqlx::{Pool, Postgres};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::panic;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

pub mod database;
pub mod datatypes;

pub async fn main(db: Arc<Pool<Postgres>>) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<DateEntry>();

    let start = NaiveDate::from_ymd_opt(2021, 5, 18).unwrap();
    let end = Utc::now().date_naive() - Duration::days(1);

    // Create a vector of all dates to process
    let mut dates: Vec<chrono::NaiveDate> = Vec::new();
    for dt in start.iter_days().take_while(|&dt| dt <= end) {
        let processed: Option<(String,)> =
            sqlx::query_as("SELECT date FROM playtime_processed WHERE date = $1")
                .bind(dt)
                .fetch_optional(db.borrow())
                .await?;

        if processed.is_some() {
            //tracing::info!("Already processed {}, skipping!", dt);
            continue;
        }
        dates.push(dt);
    }

    let writer_thread = tokio::spawn(async move {
        for mut date_entry in rx {
            match database::insert_snapshot(&mut date_entry, &db).await {
                Ok(()) => continue,
                Err(e) => {
                    tracing::error!(
                        "An error occured while inserting {}, {:?}",
                        date_entry.date,
                        e
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
                tracing::info!("{:?} - Processing {}", thread::current().id(), dt);
                let mut date_entry = DateEntry {
                    date: dt,
                    snapshot: HashMap::new(),
                };
                if database::process_day(&mut date_entry).is_ok() {
                    tx_clone.send(date_entry).unwrap()
                }
            });
            if result.is_err() {
                tracing::error!("An error occured while processing {} {:?}", dt, result)
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
