use std::time::Instant;

use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use std::{error::Error, fs::File, io::BufReader};

use csv::{Reader, ReaderBuilder};
use csv_diff::{
    csv::Csv,
    csv_diff::CsvByteDiffLocalBuilder,
    diff_result::DiffByteRecords,
    diff_row::{ByteRecordLineInfo, DiffByteRecord},
};

use models::{Map, MapInfo, Race, Teamrace};

pub mod models;

const BATCH_AMOUNT: usize = 10000;

#[derive(Clone)]
pub enum Type {
    Maps,
    Mapinfo,
    Race,
    Teamrace,
}

pub struct DiffRecordsType {
    csv_type: Type,
    diff_byte_records: DiffByteRecords,
}

fn create_diff(
    source: String,
    target: String,
    csv_type: Type,
    primary_keys: Vec<usize>,
) -> DiffRecordsType {
    let now = Instant::now();

    let differ = CsvByteDiffLocalBuilder::new()
        .primary_key_columns(primary_keys)
        .build()
        .unwrap();

    let source_reader = load_csv_file_to_reader(format!("{}/{}", "data", source)).unwrap();
    let target_reader = load_csv_file_to_reader(format!("{}/{}", "data", target)).unwrap();

    let mut diff_byte_records = differ
        .diff(
            Csv::with_reader_seek(target_reader.into_inner()),
            Csv::with_reader_seek(source_reader.into_inner()),
        )
        .unwrap();

    diff_byte_records.sort_by_line();

    let elapsed = now.elapsed();
    tracing::info!("Diffing {} and {} took: {:.2?}", source, target, elapsed);

    DiffRecordsType {
        csv_type: csv_type.clone(),
        diff_byte_records,
    }
}

async fn process_diff_add_record(
    diff_records_add: Vec<&ByteRecordLineInfo>,
    csv_type: Type,
    db: &Pool<Postgres>,
) {
    match csv_type {
        Type::Maps => {
            for diff_record_add in diff_records_add {
                let map: Map = diff_record_add.byte_record().deserialize(None).unwrap();
                let date: Option<NaiveDateTime> =
                    NaiveDateTime::parse_from_str(map.timestamp.as_str(), "%Y-%m-%d %H:%M:%S").ok();
                let _ = sqlx::query_file!(
                    "sql/database-sync/insert-map.sql",
                    map.map,
                    map.server,
                    map.points,
                    map.stars,
                    map.mapper,
                    date
                )
                .execute(db)
                .await;
            }
        }
        Type::Mapinfo => {
            for diff_record_add in diff_records_add {
                let mapinfo: MapInfo = diff_record_add.byte_record().deserialize(None).unwrap();
                let _ = sqlx::query_file!(
                    "sql/database-sync/insert-mapinfo.sql",
                    mapinfo.map,
                    mapinfo.width,
                    mapinfo.height,
                    mapinfo.death,
                    mapinfo.through,
                    mapinfo.jump,
                    mapinfo.dfreeze,
                    mapinfo.ehook_start,
                    mapinfo.hit_end,
                    mapinfo.solo_start,
                    mapinfo.tele_gun,
                    mapinfo.tele_grenade,
                    mapinfo.tele_laser,
                    mapinfo.npc_start,
                    mapinfo.super_start,
                    mapinfo.jetpack_start,
                    mapinfo.walljump,
                    mapinfo.nph_start,
                    mapinfo.weapon_shotgun,
                    mapinfo.weapon_grenade,
                    mapinfo.powerup_ninja,
                    mapinfo.weapon_rifle,
                    mapinfo.laser_stop,
                    mapinfo.crazy_shotgun,
                    mapinfo.dragger,
                    mapinfo.door,
                    mapinfo.switch_timed,
                    mapinfo.switch,
                    mapinfo.stop,
                    mapinfo.through_all,
                    mapinfo.tune,
                    mapinfo.oldlaser,
                    mapinfo.teleinevil,
                    mapinfo.telein,
                    mapinfo.telecheck,
                    mapinfo.teleinweapon,
                    mapinfo.teleinhook,
                    mapinfo.checkpoint_first,
                    mapinfo.bonus,
                    mapinfo.boost,
                    mapinfo.plasmaf,
                    mapinfo.plasmae,
                    mapinfo.plasmau
                )
                .execute(db)
                .await;
            }
        }
        Type::Race => race_batch_insert(diff_records_add, db).await,
        Type::Teamrace => teamrace_batch_insert(diff_records_add, db).await,
    }
}

async fn teamrace_batch_insert(diff_records_add: Vec<&ByteRecordLineInfo>, db: &Pool<Postgres>) {
    tracing::info!("Batch called with {}", diff_records_add.len());
    let mut map = Vec::with_capacity(BATCH_AMOUNT);
    let mut name = Vec::with_capacity(BATCH_AMOUNT);
    let mut time = Vec::with_capacity(BATCH_AMOUNT);
    let mut timestamp = Vec::with_capacity(BATCH_AMOUNT);
    let mut id = Vec::with_capacity(BATCH_AMOUNT);

    for diff_record_add in diff_records_add {
        let teamrace: Teamrace = diff_record_add.byte_record().deserialize(None).unwrap();
        map.push(teamrace.map);
        name.push(teamrace.name);
        time.push(teamrace.time);
        timestamp.push(
            NaiveDateTime::parse_from_str(teamrace.timestamp.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        );
        id.push(teamrace.id);
    }
    let _ = sqlx::query_file!(
        "sql/database-sync/insert-teamrace.sql",
        &map,
        &name,
        &time,
        &timestamp,
        &id
    )
    .execute(db)
    .await;
}

async fn race_batch_insert(diff_records_add: Vec<&ByteRecordLineInfo>, db: &Pool<Postgres>) {
    tracing::info!("Race batch insert called with {}", diff_records_add.len());

    let mut map = Vec::with_capacity(BATCH_AMOUNT);
    let mut name = Vec::with_capacity(BATCH_AMOUNT);
    let mut time = Vec::with_capacity(BATCH_AMOUNT);
    let mut timestamp = Vec::with_capacity(BATCH_AMOUNT);
    let mut server = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp1 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp2 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp3 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp4 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp5 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp6 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp7 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp8 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp9 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp10 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp11 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp12 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp13 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp14 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp15 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp16 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp17 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp18 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp19 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp20 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp21 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp22 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp23 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp24 = Vec::with_capacity(BATCH_AMOUNT);
    let mut cp25 = Vec::with_capacity(BATCH_AMOUNT);

    for diff_record_add in diff_records_add {
        let race: Race = diff_record_add.byte_record().deserialize(None).unwrap();
        map.push(race.map);
        name.push(race.name);
        time.push(race.time);
        timestamp.push(
            NaiveDateTime::parse_from_str(race.timestamp.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        server.push(race.server);
        cp1.push(race.cp1);
        cp2.push(race.cp2);
        cp3.push(race.cp3);
        cp4.push(race.cp4);
        cp5.push(race.cp5);
        cp6.push(race.cp6);
        cp7.push(race.cp7);
        cp8.push(race.cp8);
        cp9.push(race.cp9);
        cp10.push(race.cp10);
        cp11.push(race.cp11);
        cp12.push(race.cp12);
        cp13.push(race.cp13);
        cp14.push(race.cp14);
        cp15.push(race.cp15);
        cp16.push(race.cp16);
        cp17.push(race.cp17);
        cp18.push(race.cp18);
        cp19.push(race.cp19);
        cp20.push(race.cp20);
        cp21.push(race.cp21);
        cp22.push(race.cp22);
        cp23.push(race.cp23);
        cp24.push(race.cp24);
        cp25.push(race.cp25);
    }
    let _ = sqlx::query_file!(
        "sql/database-sync/insert-race.sql",
        &map,
        &name,
        &time,
        &timestamp,
        &server,
        &cp1,
        &cp2,
        &cp3,
        &cp4,
        &cp5,
        &cp6,
        &cp7,
        &cp8,
        &cp9,
        &cp10,
        &cp11,
        &cp12,
        &cp13,
        &cp14,
        &cp15,
        &cp16,
        &cp17,
        &cp18,
        &cp19,
        &cp20,
        &cp21,
        &cp22,
        &cp23,
        &cp24,
        &cp25,
    )
    .execute(db)
    .await;
}

async fn process_diff_delete_record(
    diff_record_delete: &ByteRecordLineInfo,
    csv_type: Type,
    db: &Pool<Postgres>,
) {
    match csv_type {
        Type::Maps => {
            let map: Map = diff_record_delete.byte_record().deserialize(None).unwrap();
            let _ = sqlx::query_file!("sql/database-sync/delete-map.sql", map.map,)
                .execute(db)
                .await;
        }
        Type::Mapinfo => {
            let mapinfo: MapInfo = diff_record_delete.byte_record().deserialize(None).unwrap();
            let _ = sqlx::query_file!("sql/database-sync/delete-mapinfo.sql", mapinfo.map,)
                .execute(db)
                .await;
        }
        Type::Race => {
            let race: Race = diff_record_delete.byte_record().deserialize(None).unwrap();
            let _ = sqlx::query_file!(
                "sql/database-sync/delete-race.sql",
                race.map,
                race.name,
                race.time,
                NaiveDateTime::parse_from_str(race.timestamp.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                race.server
            )
            .execute(db)
            .await;
        }
        Type::Teamrace => {
            let teamrace: Teamrace = diff_record_delete.byte_record().deserialize(None).unwrap();
            let _ = sqlx::query_file!(
                "sql/database-sync/delete-teamrace.sql",
                teamrace.map,
                teamrace.name,
                teamrace.time,
                NaiveDateTime::parse_from_str(teamrace.timestamp.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                teamrace.id
            )
            .execute(db)
            .await;
        }
    }
}

async fn process_diff_records(diff_records_type: &DiffRecordsType, db: &Pool<Postgres>) {
    let mut add_records: Vec<&ByteRecordLineInfo> = Vec::with_capacity(BATCH_AMOUNT);

    for record in diff_records_type.diff_byte_records.iter() {
        match record {
            DiffByteRecord::Add(add) => add_records.push(add),
            DiffByteRecord::Modify {
                delete,
                add,
                field_indices: _,
            } => {
                process_diff_delete_record(delete, diff_records_type.csv_type.clone(), db).await;
                add_records.push(add);
            }
            DiffByteRecord::Delete(delete) => {
                process_diff_delete_record(delete, diff_records_type.csv_type.clone(), db).await;
            }
        }
    }
    tracing::info!("Attempting insert");
    process_diff_add_record(add_records, diff_records_type.csv_type.clone(), db).await;
}

fn load_csv_file_to_reader(filename: String) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
    let reader_builder = ReaderBuilder::new();
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    Ok(reader_builder.from_reader(buf_reader))
}

fn load_csv_file_to_reader_check_empty(
    filename: String,
) -> Result<Option<Reader<BufReader<File>>>, Box<dyn Error>> {
    let reader_builder = ReaderBuilder::new();
    let file = File::open(&filename)?;
    let buf_reader = BufReader::new(file);
    let reader = reader_builder.from_reader(buf_reader);

    match reader.into_records().count() {
        0 => Ok(None),
        _ => {
            let file = File::open(&filename)?;
            let buf_reader = BufReader::new(file);
            let reader = reader_builder.from_reader(buf_reader);
            Ok(Some(reader))
        }
    }
}

pub async fn main(db: &Pool<Postgres>) {
    // Maps
    let maps_diff_records = create_diff(
        "maps.csv".to_string(),
        "maps-psql.csv".to_string(),
        Type::Maps,
        vec![0],
    );
    process_diff_records(&maps_diff_records, db).await;

    // MapInfo
    let mapinfo_diff_records = create_diff(
        "mapinfo.csv".to_string(),
        "mapinfo-psql.csv".to_string(),
        Type::Mapinfo,
        vec![0],
    );
    process_diff_records(&mapinfo_diff_records, db).await;

    // Teamrace
    let teamrace_diff_records = create_diff(
        "teamrace.csv".to_string(),
        "teamrace-psql.csv".to_string(),
        Type::Teamrace,
        vec![1, 4],
    );
    process_diff_records(&teamrace_diff_records, db).await;

    // // Race
    let race_diff_records = create_diff(
        "race.csv".to_string(),
        "race-psql.csv".to_string(),
        Type::Race,
        vec![0, 1, 2, 3, 4],
    );
    process_diff_records(&race_diff_records, db).await;

    //clean_insert("race.csv".to_string(), Type::Race, &database).await;
    //clean_insert("teamrace.csv".to_string(), Type::Teamrace, &database).await;
}

async fn clean_insert(source: String, csv_type: Type, db: &Pool<Postgres>) {
    let source_reader = load_csv_file_to_reader(format!("{}/{}", "data", source)).unwrap();
    let csv_reader = Csv::with_reader_seek(source_reader.into_inner());

    let mut records: Vec<ByteRecordLineInfo> = Vec::with_capacity(BATCH_AMOUNT);
    let mut batch_count = 0;

    for result in csv_reader.into_csv_reader().records() {
        records.push(ByteRecordLineInfo::new(
            result.unwrap().clone().into_byte_record(),
            0,
        ));
        batch_count += 1;

        // Process records in batches
        if batch_count == BATCH_AMOUNT {
            let chunk_refs: Vec<_> = records.iter().collect();

            match csv_type {
                Type::Race => race_batch_insert(chunk_refs, db).await,
                Type::Teamrace => teamrace_batch_insert(chunk_refs, db).await,
                _ => unimplemented!(), // Maps, mapinfo won't call this function anyways
            }
            // Clear the batch and reset the count
            records.clear();
            batch_count = 0;
        }
    }

    // Process any remaining records
    if !records.is_empty() {
        let chunk_refs: Vec<_> = records.iter().collect();
        match csv_type {
            Type::Race => race_batch_insert(chunk_refs, db).await,
            Type::Teamrace => teamrace_batch_insert(chunk_refs, db).await,
            _ => unimplemented!(), // Maps, mapinfo won't call this function anyways
        }
    }
}
