use mars_api_rs_macro::IdentifiableDocument;
use mars_api_rs_derive::IdentifiableDocument;
use serde::{Serialize, Deserialize};

use crate::database::CollectionOwner;

#[derive(Debug, Serialize, Deserialize, IdentifiableDocument)]
pub struct Tag {
    #[serde(rename = "_id")] 
    #[id] 
    pub id: String,
    pub name: String,
    #[serde(rename = "nameLower")]
    pub name_lower: String,
    pub display: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64
}

impl CollectionOwner<Tag> for Tag {
    fn get_collection(database: &crate::database::Database) -> &mongodb::Collection<Tag> {
        &database.tags
    }

    fn get_collection_name() -> &'static str {
        "tag"
    }
}
