use serde::{Serialize, Deserialize};

use crate::database::models::r#match::Match;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusResponse {
    pub last_alive_time: u64,
    pub current_match: Match,
    pub stats_tracking: bool
}
