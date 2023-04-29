use serde::{Serialize, Deserialize};

use crate::database::models::r#match::GoalCollection;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchLoadData {
    pub map_id: String,
    pub parties: Vec<PartyData>,
    pub goals: GoalCollection
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyData {
    pub name: String, 
    pub alias: String, 
    pub color: String, 
    pub min: u32, 
    pub max: u32
}
