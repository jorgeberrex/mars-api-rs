use mars_api_rs_derive::IdentifiableDocument;
use mars_api_rs_macro::IdentifiableDocument;
use serde::{Deserialize, Serialize};

use crate::database::CollectionOwner;

use super::player::SimplePlayer;

#[derive(Deserialize, Serialize, IdentifiableDocument, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Death {
    #[id]
    #[serde(rename = "_id")]
    pub id: String,
    pub victim: SimplePlayer,
    pub attacker: Option<SimplePlayer>,
    pub weapon: Option<String>,
    pub entity: Option<String>,
    pub distance: Option<u32>,
    pub key: String,
    pub cause: DamageCause,
    pub server_id: String,
    pub match_id: String,
    pub created_at: u64
}

impl CollectionOwner<Death> for Death {
    fn get_collection(database: &crate::database::Database) -> &mongodb::Collection<Death> {
        &database.deaths
    }

    fn get_collection_name() -> &'static str {
        "deaths"
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageCause {
    Melee,
    Projectile,
    Explosion,
    Fire,
    Lava,
    Potion,
    Flatten,
    Fall,
    Prick,
    Drown,
    Starve,
    Suffocate,
    Shock,
    Spleef,
    Void,
    Unknown
}
