use chrono::{Duration, TimeZone, Utc};
use rusqlite::types::Null;
use rusqlite::{params, Connection};
use serde::Deserialize;
use simd_json;
use std::collections::HashMap;
use std::error::Error;
use std::ptr::null;
use std::time::Instant;
use tar::Archive;

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

fn process_day(date: chrono::NaiveDate, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT date FROM processed WHERE date = ?1")?;
    let mut rows = stmt.query([date.format("%Y-%m-%d").to_string()])?;

    while let Some(_row) = rows.next()? {
        println!("Already processed, skipping!");
        return Ok(());
    }

    let resp = ureq::get(&format!(
        "https://ddnet.org/stats/master/{}.tar.zstd",
        date.format("%Y-%m-%d")
    ))
    .call()?;
    let decoder = zstd::stream::Decoder::new(resp.into_reader())?;

    let mut archive = Archive::new(decoder);
    
    let mut snapshot: HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<i32, HashMap<String, HashMap<i32, HashMap<i32, HashMap<i32, HashMap<i32, HashMap<i32, i32>>>>>>>>>>>> = HashMap::new();

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
                    let location = match server.location.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let game_type = match server.info.game_type.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let map = match server.info.map.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let name = match client.name.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let clan = match client.clan.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let country = match client.country.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let skin = match client.skin.as_ref() {
                        Some(value) => value,
                        None => null(),
                    };
                    if skin.is_null() { 
                        let location_snapshot = snapshot
                            .entry(location.clone())
                            .or_insert(HashMap::new());
                        let game_type_snapshot = location_snapshot
                            .entry(game_type.clone())
                            .or_insert(HashMap::new());
                        let map_snapshot = game_type_snapshot
                            .entry(String::from(map.name.clone()))
                            .or_insert(HashMap::new());
                        let name_snapshot = map_snapshot
                            .entry(name.clone())
                            .or_insert(HashMap::new());
                        let clan_snapshot = name_snapshot
                            .entry(clan.clone())
                            .or_insert(HashMap::new());
                        let country_snapshot = clan_snapshot
                            .entry(country.clone())
                            .or_insert(HashMap::new());
                        let skin_name_snapshot = country_snapshot
                            .entry(String::from("RUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUST"))
                            .or_insert(HashMap::new());
                        let skin_custom_colors_snapshot = skin_name_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let skin_color_body_snapshot = skin_custom_colors_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let skin_color_feet_snapshot = skin_color_body_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let afk_snapshot = skin_color_feet_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let team_snapshot = afk_snapshot
                            .entry(-69.clone())
                            .or_insert(0);
                        *team_snapshot += 5;
                        continue;
                    }
                    let skin = match client.skin.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let skin_name = match skin.name.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let afk = match client.afk.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let team = match client.team.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let skin_color_body = match skin.color_body.as_ref() {
                        Some(value) => value,
                        None => null(),
                    };
                    let skin_color_feet = match skin.color_feet.as_ref() {
                        Some(value) => value,
                        None => null(),
                    };
                    let fake_afk = if *afk {1} else {0};
                    if skin_color_body.is_null() && skin_color_feet.is_null() {
                        let location_snapshot = snapshot
                            .entry(location.clone())
                            .or_insert(HashMap::new());
                        let game_type_snapshot = location_snapshot
                            .entry(game_type.clone())
                            .or_insert(HashMap::new());
                        let map_snapshot = game_type_snapshot
                            .entry(String::from(map.name.clone()))
                            .or_insert(HashMap::new());
                        let name_snapshot = map_snapshot
                            .entry(name.clone())
                            .or_insert(HashMap::new());
                        let clan_snapshot = name_snapshot
                            .entry(clan.clone())
                            .or_insert(HashMap::new());
                        let country_snapshot = clan_snapshot
                            .entry(country.clone())
                            .or_insert(HashMap::new());
                        let skin_name_snapshot = country_snapshot
                            .entry(String::from(skin_name.clone()))
                            .or_insert(HashMap::new());
                        let skin_custom_colors_snapshot = skin_name_snapshot
                            .entry(0)
                            .or_insert(HashMap::new());
                        let skin_color_body_snapshot = skin_custom_colors_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let skin_color_feet_snapshot = skin_color_body_snapshot
                            .entry(-69.clone())
                            .or_insert(HashMap::new());
                        let afk_snapshot = skin_color_feet_snapshot
                            .entry(fake_afk.clone())
                            .or_insert(HashMap::new());
                        let team_snapshot = afk_snapshot
                            .entry(team.clone())
                            .or_insert(0);
                        *team_snapshot += 5;
                        continue;
                    }
                    let skin_color_body = match skin.color_body.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let skin_color_feet = match skin.color_feet.as_ref() {
                        Some(value) => value,
                        None => continue,
                    };
                    let location_snapshot = snapshot
                        .entry(location.clone())
                        .or_insert(HashMap::new());
                    let game_type_snapshot = location_snapshot
                        .entry(game_type.clone())
                        .or_insert(HashMap::new());
                    let map_snapshot = game_type_snapshot
                        .entry(String::from(map.name.clone()))
                        .or_insert(HashMap::new());
                    let name_snapshot = map_snapshot
                        .entry(name.clone())
                        .or_insert(HashMap::new());
                    let clan_snapshot = name_snapshot
                        .entry(clan.clone())
                        .or_insert(HashMap::new());
                    let country_snapshot = clan_snapshot
                        .entry(country.clone())
                        .or_insert(HashMap::new());
                    let skin_name_snapshot = country_snapshot
                        .entry(String::from(skin_name.clone()))
                        .or_insert(HashMap::new());
                    let skin_custom_colors_snapshot = skin_name_snapshot
                        .entry(1)
                        .or_insert(HashMap::new());
                    let skin_color_body_snapshot = skin_custom_colors_snapshot
                        .entry(skin_color_body.clone())
                        .or_insert(HashMap::new());
                    let skin_color_feet_snapshot = skin_color_body_snapshot
                        .entry(skin_color_feet.clone())
                        .or_insert(HashMap::new());
                    let afk_snapshot = skin_color_feet_snapshot
                        .entry(fake_afk.clone())
                        .or_insert(HashMap::new());
                    let team_snapshot = afk_snapshot
                        .entry(team.clone())
                        .or_insert(0);
                    *team_snapshot += 5;
                }
            }
        }
    }
    let duration = time.elapsed();
    println!("Parsing took: {:?}", duration);

    let time = Instant::now();
    for (location, location_snapshot) in snapshot {
        for (gametype, game_type_snapshot) in location_snapshot {
            for (map, map_snapshot) in game_type_snapshot {
                for (name, name_snapshot) in map_snapshot {
                    for (clan, clan_snapshot) in name_snapshot {
                        for (country, country_snapshot) in clan_snapshot {
                            for (skin_name, skin_name_snapshot) in country_snapshot {
                                for (skin_custom_colors, skin_custom_colors_snapshot) in skin_name_snapshot {
                                    for (skin_color_body, skin_color_body_snapshot) in skin_custom_colors_snapshot {
                                        for (skin_color_feet, skin_color_feet_snapshot) in skin_color_body_snapshot {
                                            for (afk, afk_snapshot) in skin_color_feet_snapshot {
                                                for (team, time) in afk_snapshot {
                                                    if skin_name == "RUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUST" {
                                                        let stmt = "INSERT INTO record_snapshot (date, location, gametype, map, name, clan, country, skin_name, skin_custom_colors, skin_color_body, skin_color_feet, afk, team, time) VALUES (?1 ,?2 ,?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);";
                                                        conn.execute(
                                                            stmt,
                                                            (
                                                                date.format("%Y-%m-%d").to_string(),
                                                                location.clone(),
                                                                gametype.clone(),
                                                                map.clone(),
                                                                name.clone(),
                                                                clan.clone(),
                                                                country.clone(),
                                                                Null,
                                                                Null,
                                                                Null,
                                                                Null,
                                                                Null,
                                                                Null,
                                                                Null
                                                            ),
                                                        )
                                                        .unwrap();
                                                        continue;
                                                    }
                                                    if skin_custom_colors == 0 {
                                                        let stmt = "INSERT INTO record_snapshot (date, location, gametype, map, name, clan, country, skin_name, skin_custom_colors, skin_color_body, skin_color_feet, afk, team, time) VALUES (?1 ,?2 ,?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);";
                                                        conn.execute(
                                                            stmt,
                                                            (
                                                                date.format("%Y-%m-%d").to_string(),
                                                                location.clone(),
                                                                gametype.clone(),
                                                                map.clone(),
                                                                name.clone(),
                                                                clan.clone(),
                                                                country.clone(),
                                                                skin_name.clone(),
                                                                skin_custom_colors.clone(),
                                                                Null,
                                                                Null,
                                                                afk.clone(),
                                                                team.clone(),
                                                                time.clone()
                                                            ),
                                                        )
                                                        .unwrap();
                                                        continue;
                                                    }
                                                    let stmt = "INSERT INTO record_snapshot (date, location, gametype, map, name, clan, country, skin_name, skin_custom_colors, skin_color_body, skin_color_feet, afk, team, time) VALUES (?1 ,?2 ,?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);";
                                                    conn.execute(
                                                        stmt,
                                                        (
                                                            date.format("%Y-%m-%d").to_string(),
                                                            location.clone(),
                                                            gametype.clone(),
                                                            map.clone(),
                                                            name.clone(),
                                                            clan.clone(),
                                                            country.clone(),
                                                            skin_name.clone(),
                                                            skin_custom_colors.clone(),
                                                            skin_color_body.clone(),
                                                            skin_color_feet.clone(),
                                                            afk.clone(),
                                                            team.clone(),
                                                            time.clone()
                                                        ),
                                                    )
                                                    .unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let duration = time.elapsed();
    println!("Inserting took: {:?}", duration);

    conn.execute(
        "INSERT INTO processed (date) VALUES (?1)",
        params![date.format("%Y-%m-%d").to_string()],
    )
    .unwrap();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("../db/master.db").unwrap();

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
                    skin_custom_colors INTEGER,
                    skin_color_body INTEGER,
                    skin_color_feet INTEGER,
                    afk INTEGER,
                    team INTEGER,
                    time INTEGER)",
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
        .with_ymd_and_hms(2021, 05, 18, 0, 0, 0)
        .single()
        .unwrap()
        .date_naive();
    let end = Utc::now().date_naive() - Duration::days(1);

    let total_days = (end - start).num_days() + 1;

    for (i, dt) in start.iter_days().take_while(|&dt| dt <= end).enumerate() {
        println!("{} [{}/{}]", dt, i + 1, total_days);
        process_day(dt, &conn).unwrap();
    }

    Ok(())
}
