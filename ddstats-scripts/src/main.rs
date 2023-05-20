use chrono::{Duration, TimeZone, Utc, Date};
use serde::Deserialize;
use std::error::Error;
use std::{collections::HashMap};
use tar::Archive;
use simd_json;
use rusqlite::{Connection, params};

#[derive(Debug, Deserialize, Clone)]
struct Client {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Map>,
    pub clients: Option<Vec<Client>>,
}

#[derive(Debug, Deserialize)]
struct Map {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct Server {
    pub info: Info,
}

#[derive(Debug, Deserialize)]
struct ServerList {
    pub servers: Vec<Server>,
}

fn process_day(date: chrono::Date<Utc>, conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Check if date is processed (ugly as shit)
    let mut stmt = conn.prepare("SELECT date FROM processed WHERE date = ?1")?;
    let mut rows = stmt.query([date.format("%Y-%m-%d").to_string()])?;

    while let Some(_row) = rows.next()? {
        println!("Already processed, skipping!");
        return Ok(())
    }
    
    let resp = ureq::get(&format!("https://ddnet.org/stats/master/{}.tar.zstd", date.format("%Y-%m-%d"))).call()?;
    let decoder = zstd::stream::Decoder::new(resp.into_reader())?;

    let mut playtime: HashMap<String, HashMap<String, i64>> = HashMap::new();

    let mut archive = Archive::new(decoder);

    // Loop over the entries
    for entry in archive.entries()? {
        let entry = entry.unwrap();
        let path = entry.path().unwrap();
        let filename = path.file_name().expect("exist");
        println!("File: {}", filename.to_string_lossy());

        // S-Tier error handling
        let data: ServerList = match simd_json::from_reader(entry) {
            Ok(data) => data,
            Err(_) => continue,
        };

        for server in data.servers.iter() {
            for clients in server.info.clients.iter() {
                for client in clients.iter() {
                    let player = playtime.entry(String::from(client.name.clone())).or_insert(HashMap::new());
                    let map = server.info.map.as_ref().unwrap();

                    let map = player.entry(String::from(map.name.clone())).or_insert(0);
                    *map += 5;
                }
            }
        }
    }

    let mut rows = 0;
    for player in playtime.iter() {
        rows += player.1.keys().len();
    }
    print!("Rows: {}\n", rows);

    // Attempt to insert
    for player in playtime.iter() {
        for map in player.1.iter() {
            let stmt = "INSERT INTO record_playtime (date, player, map, time) VALUES (?1 ,?2 ,?3, ?4);";
            conn.execute(stmt, (date.format("%Y-%m-%d").to_string(), player.0, map.0, map.1)).unwrap();
        }
    }
    // Mark the date as processed to avoid duplicates
    conn.execute("INSERT INTO processed (date) VALUES (?1)",
    params![date.format("%Y-%m-%d").to_string()]).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the database
    let conn = Connection::open("../playtime.db").unwrap();

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
                map CHAR(128) NOT NULL,
                player CHAR(32) NOT NULL,
                time INTEGER not null)",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed (
            date TEXT)",
        [],
    ).unwrap();

    let start = Utc.ymd(2021, 05, 18);
    let end = Utc::now().date() - Duration::days(1);

    let mut dt = start;
    while dt <= end {
        println!("{:?}", dt);
        process_day(dt, &conn).unwrap();
        dt = dt + Duration::days(1);
    }
    
    Ok(())
}