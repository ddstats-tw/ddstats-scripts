use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub map: String,
    pub server: String,
    pub points: i32,
    pub stars: i32,
    pub mapper: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapInfo {
    pub map: String,
    pub width: i32,
    pub height: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub death: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub through: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub jump: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub dfreeze: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub ehook_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub hit_end: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub solo_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub tele_gun: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub tele_grenade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub tele_laser: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub npc_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub super_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub jetpack_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub walljump: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nph_start: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapon_shotgun: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapon_grenade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub powerup_ninja: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapon_rifle: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub laser_stop: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub crazy_shotgun: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub dragger: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub door: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub switch_timed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub switch: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub stop: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub through_all: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub tune: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub oldlaser: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub teleinevil: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub telein: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub telecheck: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub teleinweapon: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub teleinhook: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub checkpoint_first: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub bonus: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub boost: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub plasmaf: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub plasmae: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub plasmau: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Race {
    pub map: String,
    pub name: String,
    pub time: f64,
    pub timestamp: String,
    pub server: String,
    pub cp1: f64,
    pub cp2: f64,
    pub cp3: f64,
    pub cp4: f64,
    pub cp5: f64,
    pub cp6: f64,
    pub cp7: f64,
    pub cp8: f64,
    pub cp9: f64,
    pub cp10: f64,
    pub cp11: f64,
    pub cp12: f64,
    pub cp13: f64,
    pub cp14: f64,
    pub cp15: f64,
    pub cp16: f64,
    pub cp17: f64,
    pub cp18: f64,
    pub cp19: f64,
    pub cp20: f64,
    pub cp21: f64,
    pub cp22: f64,
    pub cp23: f64,
    pub cp24: f64,
    pub cp25: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Teamrace {
    pub map: String,
    pub name: String,
    pub time: f64,
    pub timestamp: String,
    pub id: String,
}
