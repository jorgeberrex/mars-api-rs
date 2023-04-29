use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JoinSound {
    pub id: String,
    pub name: String,
    pub description: Vec<String>,
    pub sound: String,
    pub permission: String,
    pub gui_icon: String,
    pub gui_slot: u32,
    pub volume: f64,
    pub pitch: f64
}