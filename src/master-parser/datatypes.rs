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
}

impl Client {
    pub fn process(client: &Client, server: &Server, snapshot: &mut SnapshotStore) {
        // required values
        let name = client.name.to_string();

        // exclude some common names
        if name == "(connecting)"
            || name == "(1)"
            || name == "."
            || name == "brainless tee"
            || name.ends_with("nameless tee")
            || name.ends_with(")nameless te")
            || name.ends_with("brainless te")
        {
            return;
        }

        let location = server.location.to_string();
        let game_type = server.info.game_type.to_string();
        let map = server.info.map.name.clone().to_string();
        let clan = client.clan.to_string();
        let country = client.country;

        // length checks
        if game_type.len() > 32 || map.len() > 128 || clan.len() >= 12 || name.len() >= 16 {
            return;
        }

        // optional values
        let skin_name = client.skin.clone().and_then(|s| s.name);
        let skin_color_body = client
            .skin
            .as_ref()
            .and_then(|s| s.color_body.map(|color| color.clamp(0, 0xffffff)));
        let skin_color_feet = client
            .skin
            .as_ref()
            .and_then(|s| s.color_feet.map(|color| color.clamp(0, 0xffffff)));

        if skin_name.is_some() && skin_name.clone().unwrap().len() >= 24 {
            return;
        }

        // Create a key based on the extracted values
        let key = SnapshotKey {
            location,
            game_type,
            map,
            name,
            clan,
            country,
            skin_name,
            skin_color_body,
            skin_color_feet,
        };

        // Insert or update the snapshot entry
        let counter = snapshot.entry(key).or_insert(0);
        *counter += 5;
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub map: Map,
    pub game_type: String,
    pub clients: Option<Vec<Client>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Map {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub info: Info,
    #[serde(default = "default_location")]
    pub location: String,
}

fn default_location() -> String {
    "unknown".to_string()
}

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct ServerList {
    #[serde_as(as = "serde_with::VecSkipError<_>")]
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
}

pub type SnapshotStore = HashMap<SnapshotKey, i32>;

pub struct DateEntry {
    pub snapshot: SnapshotStore,
    pub date: chrono::NaiveDate,
}
