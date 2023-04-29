use mars_api_rs_derive::IdentifiableDocument;
use mars_api_rs_macro::IdentifiableDocument;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};

use crate::database::{CollectionOwner, Database};

#[derive(Deserialize, Serialize, Debug, IdentifiableDocument)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    #[id]
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub name_lower: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub prefix: Option<String>,
    pub priority: u32,
    pub permissions: Vec<String>,
    pub staff: bool,
    pub apply_on_join: bool,
    pub created_at: f64
}

impl CollectionOwner<Rank> for Rank {
    fn get_collection(database: &crate::database::Database) -> &mongodb::Collection<Rank> {
        &database.ranks
    }

    fn get_collection_name() -> &'static str {
        "ranks"
    }
}

impl Rank {
    pub async fn find_default(database: &Database) -> Vec<Rank> {
        let cursor = match Rank::get_collection(database).find(doc! {
            "applyOnJoin": true
        }, None).await {
            Ok(ranks_cursor) => ranks_cursor,
            Err(_) => return Vec::new()
        };
        Database::consume_cursor_into_owning_vec(cursor).await
    }
}
