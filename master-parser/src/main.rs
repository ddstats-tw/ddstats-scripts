use chrono::{Duration, TimeZone, Utc};
use datatypes::DateEntry;
use rusqlite::Connection;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;

pub mod database;
pub mod datatypes;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("../db/master.db").unwrap();
    let (tx, rx) = channel::<DateEntry>();

    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    )
    .expect("PRAGMA");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS record_snapshot (
                    date TEXT NOT NULL,
                    location CHAR(32) NOT NULL,
                    gametype CHAR(32) NOT NULL,
                    map CHAR(128) NOT NULL,
                    name CHAR(32) NOT NULL,
                    clan CHAR(32) NOT NULL,
                    country INTEGER NOT NULL,
                    skin_name CHAR(32),
                    skin_color_body INTEGER,
                    skin_color_feet INTEGER,
                    afk INTEGER,
                    team INTEGER,
                    time INTEGER NOT NULL)",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed (
            date TEXT)",
        [],
    )
    .unwrap();

    let start = Utc
        .with_ymd_and_hms(2021, 5, 18, 0, 0, 0)
        .single()
        .unwrap()
        .date_naive();
    let end = Utc::now().date_naive() - Duration::days(1);

    // Create a vector of all dates to process
    let mut dates: Vec<chrono::NaiveDate> = Vec::new();
    for dt in start.iter_days().take_while(|&dt| dt <= end) {
        let mut is_already_processed_stmt =
            conn.prepare("SELECT date FROM processed WHERE date = ?")?;

        let date_string = dt.format("%Y-%m-%d").to_string();
        let is_processed = is_already_processed_stmt
            .query_row([&date_string], |_| Ok(()))
            .is_ok();

        if is_processed {
            println!("Already processed {}, skipping!", dt);
            continue;
        }
        dates.push(dt);
    }

    let writer_thread = thread::spawn(move || {
        for mut date_entry in rx {
            database::insert_snapshot(&mut date_entry, &conn);
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
            println!("{:?} - Processing {}", thread::current().id(), dt);
            let mut date_entry = DateEntry {
                date: dt,
                snapshot: HashMap::new(),
            };
            let _ = database::process_day(&mut date_entry);
            tx_clone.send(date_entry).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed");
    }

    // Wait for all threads to complete
    drop(tx);
    writer_thread.join().unwrap();
    Ok(())
}
