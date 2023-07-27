use chrono::{Duration, TimeZone, Utc};
use std::time::{Instant};
use serde::Deserialize;
use std::error::Error;
use std::{collections::HashMap};
use tar::Archive;
use simd_json;
use rusqlite::{Connection, params};

#[derive(Debug, Deserialize, Clone)]
struct Client {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

fn process_day(date: chrono::NaiveDate, conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Check if date is processed (ugly as shit)
    let mut stmt = conn.prepare("SELECT date FROM processed WHERE date = ?1")?;
    let mut rows = stmt.query([date.format("%Y-%m-%d").to_string()])?;

    while let Some(_row) = rows.next()? {
        println!("Already processed, skipping!");
        return Ok(())
    }
    
    let resp = ureq::get(&format!("https://ddnet.org/stats/master/{}.tar.zstd", date.format("%Y-%m-%d"))).call()?;
    let decoder = zstd::stream::Decoder::new(resp.into_reader())?;

    let mut playtime: HashMap<String, HashMap<String, HashMap<String, HashMap<String, i64>>>> = HashMap::new();

    let mut archive = Archive::new(decoder);

    let start = Instant::now();
    // Loop over the entries
    for entry in archive.entries()? {
        let entry = entry.unwrap();

        // S-Tier error handling
        let data: ServerList = match simd_json::from_reader(entry) {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                continue
            }
        };

        for server in data.servers.iter() {
            for clients in server.info.clients.iter() {
                for client in clients.iter() {
                    // player , location , gamemode , map , time (i64)
                    let location = match server.location.as_ref() {
                        Some(value) => value,
                        None => continue
                    };
                    let game_type = match server.info.game_type.as_ref() {
                        Some(value) => value,
                        None => continue
                    };
                    let map = match server.info.map.as_ref() {
                        Some(value) => value,
                        None => continue
                    };
                    let name = match client.name.as_ref() {
                        Some(value) => value,
                        None => continue
                    };

                    let player_playtime = playtime.entry(name.clone()).or_insert(HashMap::new());
                    let region_playtime = player_playtime.entry(location.clone()).or_insert(HashMap::new());
                    let gamemode_playtime = region_playtime.entry(game_type.clone()).or_insert(HashMap::new());

                    let map = gamemode_playtime.entry(String::from(map.name.clone())).or_insert(0);
                    *map += 5;
                }
            }
        }
    }
    let mut rows = 0;

    // Attempt to insert
    for (player, player_playtime) in playtime {
        for (location, location_playtime) in player_playtime {
            for (gametype, gametype_playtime) in location_playtime {
                for(map, map_playtime) in gametype_playtime {
                    let stmt = "INSERT INTO record_playtime (date, player, location, gametype, map, time) VALUES (?1 ,?2 ,?3, ?4, ?5, ?6);";
                    conn.execute(stmt, (date.format("%Y-%m-%d").to_string(), 
                        player.clone(), location.clone(), gametype.clone(), map, map_playtime)).unwrap();
                    rows += 1;
                }
            }
        }
    }
    print!("Inserted {} rows!\n", rows);

    // Mark the date as processed to avoid duplicates
    conn.execute("INSERT INTO processed (date) VALUES (?1)",
    params![date.format("%Y-%m-%d").to_string()]).unwrap();
    let duration = start.elapsed();
    println!("Day took {:?}", duration);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the database
    let conn = Connection::open("../db/playtime.db").unwrap();

    // Database options
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    ).expect("PRAGMA");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS record_playtime (
                date TEXT NOT NULL,
                player CHAR(32) NOT NULL,
                location CHAR(32) NOT NULL,
                gametype CHAR(32) NOT NULL,
                map CHAR(128) NOT NULL,
                time INTEGER not null)",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed (
            date TEXT)",
        [],
    ).unwrap();

    let start = Utc.with_ymd_and_hms(2021, 05, 18, 0, 0, 0).single().unwrap().date_naive();
    let end = Utc::now().date_naive() - Duration::days(1);

    let total_days = (end - start).num_days() + 1;

    for (i, dt) in start.iter_days().take_while(|&dt| dt <= end).enumerate() {
        println!("{} [{}/{}]", dt, i + 1, total_days);
        process_day(dt, &conn).unwrap();
    }
    
    Ok(())
}
