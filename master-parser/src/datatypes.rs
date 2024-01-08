use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Skin {
    pub name: Option<String>,
    pub color_body: Option<i32>,
    pub color_feet: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Client {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub skin: Option<Skin>,
    pub afk: Option<bool>,
    pub team: Option<i32>,
}

impl Client {
    pub fn process(client: &Client, server: &Server, snapshot: &mut SnapshotStore) {
        // required values
        let location = server.location.to_string();
        let game_type = server.info.game_type.to_string();
        let map = server.info.map.clone();
        let name = client.name.to_string();
        let clan = client.clan.to_string();
        let country = client.country;

        // optional values
        let skin_name = client.skin.clone().and_then(|s| s.name);
        let skin_color_body = client.skin.as_ref().and_then(|s| s.color_body);
        let skin_color_feet = client.skin.as_ref().and_then(|s| s.color_feet);
        let afk = client.afk;
        let team = client.team;

        // Create a key based on the extracted values
        let key = SnapshotKey {
            location,
            game_type,
            map: map.name,
            name,
            clan,
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
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub map: Map,
    pub clients: Vec<Client>,
    pub game_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Map {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub location: String,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct ServerList {
    pub servers: Vec<Server>,
}

#[derive(Hash, Eq, PartialEq)]
pub struct SnapshotKey {
    pub location: String,
    pub game_type: String,
    pub map: String,
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub skin_name: Option<String>,
    pub skin_color_body: Option<i32>,
    pub skin_color_feet: Option<i32>,
    pub afk: Option<bool>,
    pub team: Option<i32>,
}

pub type SnapshotStore = HashMap<SnapshotKey, i32>;

pub struct DateEntry {
    pub snapshot: SnapshotStore,
    pub date: chrono::NaiveDate,
}
