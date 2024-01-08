use crate::datatypes::{Client, DateEntry, ServerList};
use rusqlite::{params, Connection};
use std::{error::Error, thread, time::Instant};
use tar::Archive;

pub fn insert_snapshot(date_entry: &mut DateEntry, conn: &Connection) {
    let time = Instant::now();

    //conn.execute_batch("BEGIN TRANSACTION;").unwrap();
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
    //conn.execute_batch("COMMIT TRANSACTION;").unwrap();

    // mark day as processed
    conn.execute(
        "INSERT INTO processed (date) VALUES (?1)",
        params![date_entry.date.format("%Y-%m-%d").to_string()],
    )
    .unwrap();

    let duration = time.elapsed();
    println!(
        "{:?} - Inserting {} took: {:?}",
        thread::current().id(),
        date_entry.date,
        duration
    );
}

pub fn process_day(date_entry: &mut DateEntry) -> Result<(), Box<dyn Error>> {
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
            for client in server.info.clients.iter() {
                Client::process(client, server, &mut date_entry.snapshot)
            }
        }
    }
    let duration = time.elapsed();
    println!(
        "{:?} - Parsing {} took: {:?}",
        thread::current().id(),
        date_entry.date,
        duration
    );
    Ok(())
}
