use chrono::{Duration, TimeZone, Utc};
use rusqlite::{params, Connection};
use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use tar::Archive;
use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, Deserialize, Clone)]
struct Skin {
    pub name: Option<String>,
    pub color_body: Option<i32>,
    pub color_feet: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
struct Client {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub clan: Option<String>,
    pub country: Option<i32>,
    pub skin: Option<Skin>,
    pub afk: Option<bool>,
    pub team: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Map>,
    pub clients: Option<Vec<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Map {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
struct ServerList {
    pub servers: Vec<Server>,
}

#[derive(Hash, Eq, PartialEq)]
struct SnapshotKey {
    location: String,
    game_type: String,
    map: String,
    name: String,
    clan: String,
    country: i32,
    skin_name: Option<String>,
    skin_color_body: Option<i32>,
    skin_color_feet: Option<i32>,
    afk: Option<bool>,
    team: Option<i32>,
}

type SnapshotType = HashMap<SnapshotKey, i32>;

struct DateEntry {
    snapshot: SnapshotType,
    date: chrono::NaiveDate,
}

fn process_client(client: &Client, server: &Server, snapshot: &mut SnapshotType) {
    // required values
    let location = if let Some(location) = &server.location {
        location
    } else {
        return;
    };

    let game_type = if let Some(game_type) = &server.info.game_type {
        game_type
    } else {
        return;
    };

    let map = if let Some(map) = &server.info.map {
        map
    } else {
        return;
    };

    let name = if let Some(name) = &client.name {
        name
    } else {
        return;
    };

    let clan = if let Some(clan) = &client.clan {
        clan
    } else {
        return;
    };

    let country = if let Some(country) = client.country {
        country
    } else {
        return;
    };

    // optional values
    let skin_name = client.skin.clone().and_then(|s| s.name);
    let skin_color_body = client.skin.as_ref().and_then(|s| s.color_body);
    let skin_color_feet = client.skin.as_ref().and_then(|s| s.color_feet);
    let afk = client.afk;
    let team = client.team;

    // Create a key based on the extracted values
    let key = SnapshotKey {
        location: location.to_string(),
        game_type: game_type.to_string(),
        map: map.name.to_string(),
        name: name.to_string(),
        clan: clan.to_string(),
        country,
        skin_name,
        skin_color_body,
        skin_color_feet,
        afk,
        team,
    };

    // Insert or update the snapshot entry
    let counter = snapshot.entry(key).or_insert(0);
    *counter += 5;
}

fn insert_snapshot(date_entry: &mut DateEntry, conn: &Connection) {
    let time = Instant::now();

    let mut stmt = conn.prepare("INSERT INTO record_snapshot (date, location, gametype, map, name, clan, country, skin_name, skin_color_body, skin_color_feet, afk, team, time) VALUES (?1 ,?2 ,?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13);").unwrap();
    for (key, time) in date_entry.snapshot.iter() {
        let params = (
            date_entry.date.format("%Y-%m-%d").to_string(),
            key.location.clone(),
            key.game_type.clone(),
            key.map.clone(),
            key.name.clone(),
            key.clan.clone(),
            key.country,
            key.skin_name.clone(),
            key.skin_color_body,
            key.skin_color_feet,
            key.afk,
            key.team,
            *time,
        );

        let _ = stmt.execute(params);
    }

    // mark day as processed
    conn.execute(
        "INSERT INTO processed (date) VALUES (?1)",
        params![date_entry.date.format("%Y-%m-%d").to_string()],
    ).unwrap();

    let duration = time.elapsed();
    println!("{:?} - Inserting {} took: {:?}", thread::current().id(), date_entry.date, duration);
}

fn process_day(date_entry: &mut DateEntry) -> Result<(), Box<dyn Error>> {
    let resp = ureq::get(&format!(
        "https://ddnet.org/stats/master/{}.tar.zstd",
        date_entry.date.format("%Y-%m-%d")
    ))
    .call()?;
    let decoder = zstd::stream::Decoder::new(resp.into_reader())?;

    let mut archive = Archive::new(decoder);

    let time = Instant::now();
    for entry in archive.entries()? {
        let entry = entry.unwrap();

        let data: ServerList = match simd_json::from_reader(entry) {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };

        for server in data.servers.iter() {
            for clients in server.info.clients.iter() {
                for client in clients.iter() {
                    process_client(client, server, &mut date_entry.snapshot)
                }
            }
        }
    }
    let duration = time.elapsed();
    println!("{:?} - Parsing {} took: {:?}", thread::current().id(), date_entry.date, duration);
    Ok(())
}

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
    let mut dates: Vec<chrono::NaiveDate<>> = Vec::new();
    for dt in start.iter_days().take_while(|&dt| dt <= end) {
        let mut stmt = conn.prepare("SELECT date FROM processed WHERE date = ?")?;
        let date_str = dt.format("%Y-%m-%d").to_string();
        let mut rows = stmt.query(&[&date_str])?;

        if let Some(_row) = rows.next()? {
            println!("Already processed {}, skipping!", dt);
            continue;
        }
        dates.push(dt);
    }

    let writer_thread = thread::spawn(move || {
        for mut date_entry in rx {
            insert_snapshot(&mut date_entry, &conn);
        }
    });

    const MAX_THREADS: usize = 8;

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(MAX_THREADS);
    for dt in dates {
        if handles.len() >= MAX_THREADS {
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
            let _ = process_day(&mut date_entry);
            tx_clone.send(date_entry).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed");
    }

    // Wait for all threads to complete
    writer_thread.join().unwrap();
    Ok(())
}
