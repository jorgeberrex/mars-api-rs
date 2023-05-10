use serde::{Serialize, Deserialize};

use crate::{database::models::{r#match::Match, player::SimplePlayer, server::XPMultiplier}, util::time::get_u64_time_millis};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusResponse {
    pub last_alive_time: u64,
    pub current_match: Match,
    pub stats_tracking: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XPMultiplierRequest {
    pub value: f32,
    pub player: Option<SimplePlayer>
}

impl XPMultiplierRequest {
    pub fn to_xp_multiplier(&self) -> XPMultiplier {
        XPMultiplier {
            value: self.value,
            player: self.player.clone(),
            updated_at: get_u64_time_millis()
        }
    }
}
