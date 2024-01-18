use crate::datatypes::{Client, DateEntry, ServerList};
use sqlx::{Executor, Pool, Postgres};
use std::{error::Error, thread, time::Instant};
use tar::Archive;

pub async fn insert_snapshot(
    date_entry: &mut DateEntry,
    conn: &Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let time = Instant::now();

    let mut date = Vec::new();
    let mut location = Vec::new();
    let mut game_type = Vec::new();
    let mut map = Vec::new();
    let mut name = Vec::new();
    let mut clan = Vec::new();
    let mut country = Vec::new();
    let mut skin_name = Vec::new();
    let mut skin_color_body = Vec::new();
    let mut skin_color_feet = Vec::new();
    let mut afk = Vec::new();
    let mut team = Vec::new();
    let mut times = Vec::new();

    for (key, time) in date_entry.snapshot.iter() {
        date.push(date_entry.date.format("%Y-%m-%d").to_string());
        location.push(key.location.clone());
        game_type.push(key.game_type.clone());
        map.push(key.map.clone());
        name.push(key.name.clone());
        clan.push(key.clan.clone());
        country.push(key.country);
        skin_name.push(key.skin_name.clone());
        skin_color_body.push(key.skin_color_body);
        skin_color_feet.push(key.skin_color_feet);
        afk.push(key.afk);
        team.push(key.team);
        times.push(*time);
    }

    let insert_query = r"
    INSERT INTO playtime (date, location, gametype, map, name, clan, country, skin_name, skin_color_body, skin_color_feet, afk, team, time)
    SELECT * FROM UNNEST(
        $1::TEXT[],
        $2::VARCHAR(8)[],
        $3::VARCHAR(32)[],
        $4::VARCHAR(128)[],
        $5::VARCHAR(15)[],
        $6::VARCHAR(15)[],
        $7::INTEGER[],
        $8::VARCHAR(32)[],
        $9::INTEGER[],
        $10::INTEGER[],
        $11::BOOLEAN[],
        $12::INTEGER[],
        $13::INTEGER[]
    )";

    sqlx::query(insert_query)
        .bind(date)
        .bind(location)
        .bind(game_type)
        .bind(map)
        .bind(name)
        .bind(clan)
        .bind(country)
        .bind(skin_name)
        .bind(skin_color_body)
        .bind(skin_color_feet)
        .bind(afk)
        .bind(team)
        .bind(times)
        .execute(conn)
        .await?;

    // mark day as processed
    conn.execute(
        sqlx::query("INSERT INTO playtime_processed (date) VALUES ($1)")
            .bind(date_entry.date.format("%Y-%m-%d").to_string()),
    )
    .await?;

    let duration = time.elapsed();
    println!(
        "{:?} - Inserting {} took: {:?}",
        thread::current().id(),
        date_entry.date,
        duration
    );
    Ok(())
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
            for clients in server.info.clients.iter() {
                for client in clients.iter() {
                    Client::process(client, server, &mut date_entry.snapshot)
                }
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
