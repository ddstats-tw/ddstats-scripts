use std::{collections::HashMap, time::Instant};

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Map {
    pub map: String,
    pub server: String,
    pub points: i32,
    pub stars: i32,
    pub mapper: String,
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Points {
    pub name: String,
    pub points: Option<i64>,
    pub time: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TotalPoints {
    pub rank_points: i64,
    pub team_points: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeCache {
    pub time: f64,
    pub time_team: f64,
}

async fn process_date(
    date: NaiveDate,
    maps: &Vec<Map>,
    time_cache: &mut HashMap<String, TimeCache>,
    db: &Pool<Postgres>,
) {
    let mut players: HashMap<String, TotalPoints> = HashMap::new();

    let timer = Instant::now();
    for map in maps {
        if (map
            .timestamp
            .unwrap_or(NaiveDate::from_ymd_opt(2013, 7, 18).unwrap().into()))
            > date.into()
        {
            continue;
        }

        let map_cached_time = time_cache.entry(map.map.clone()).or_insert(TimeCache {
            time: 10.0 * 100000.0,
            time_team: 10.0 * 100000.0,
        });

        let rankpoints = sqlx::query_file_as!(
            Points,
            "sql/rankpoints.sql",
            map.map,
            date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            map_cached_time.time
        )
        .fetch_all(db)
        .await
        .unwrap();

        let teampoints = sqlx::query_file_as!(
            Points,
            "sql/teampoints.sql",
            map.map,
            date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            map_cached_time.time_team
        )
        .fetch_all(db)
        .await
        .unwrap();

        if rankpoints.last().is_some() {
            let last = rankpoints.last().unwrap();
            if last.points.is_none() {
                map_cached_time.time = last.time;
            }
        }

        if teampoints.last().is_some() {
            let last = teampoints.last().unwrap();
            if last.points.is_none() {
                map_cached_time.time_team = last.time;
            }
        }

        for entry in rankpoints.iter() {
            if entry.points.is_none() {
                continue;
            }

            let points = players.entry(entry.name.clone()).or_insert(TotalPoints {
                rank_points: 0,
                team_points: 0,
            });
            points.rank_points += entry.points.unwrap();
        }

        for entry in teampoints.iter() {
            if entry.points.is_none() {
                continue;
            }

            let points = players.entry(entry.name.clone()).or_insert(TotalPoints {
                rank_points: 0,
                team_points: 0,
            });
            points.team_points += entry.points.unwrap();
        }
    }

    let mut dates = Vec::new();
    let mut names = Vec::new();
    let mut rank_points = Vec::new();
    let mut team_points = Vec::new();

    for (name, total_points) in players.iter() {
        dates.push(date);
        names.push(name.clone());
        rank_points.push(total_points.rank_points as i32);
        team_points.push(total_points.team_points as i32);
    }

    let _ = sqlx::query_file!(
        "sql/insert-rankedpoints.sql",
        &dates,
        &names,
        &rank_points,
        &team_points
    )
    .execute(db)
    .await;

    let _ = sqlx::query!(
        "INSERT INTO rankedpoints_processed (date) VALUES ($1)",
        date
    )
    .execute(db)
    .await;

    tracing::info!("Finished processing {} in {:?}", date, timer.elapsed());
}

pub async fn main(db: &Pool<Postgres>) {
    // JUST USE FROM YMD????
    let start = NaiveDate::from_ymd_opt(2013, 7, 18).unwrap();

    let end = Utc::now().date_naive() - Duration::days(1);

    let mut dates: Vec<chrono::NaiveDate> = Vec::new();
    for dt in start.iter_days().take_while(|&dt| dt <= end) {
        let processed: Option<(NaiveDate,)> =
            sqlx::query_as("SELECT date FROM rankedpoints_processed WHERE date = $1")
                .bind(dt)
                .fetch_optional(db)
                .await
                .unwrap();

        if processed.is_some() {
            //tracing::info!("Already processed {}, skipping!", dt);
            continue;
        }
        dates.push(dt);
    }

    let maps = sqlx::query_as!(
        Map,
        "SELECT map, server, points, stars, mapper, timestamp FROM maps WHERE server != 'Fun'"
    )
    .fetch_all(db)
    .await
    .unwrap();

    let mut time_cache: HashMap<String, TimeCache> = HashMap::new();
    for dt in dates {
        tracing::info!("Proccessing {}", dt);
        process_date(dt, &maps, &mut time_cache, db).await;
    }
}
