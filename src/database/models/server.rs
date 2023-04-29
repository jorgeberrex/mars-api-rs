use serde::{Serialize, Deserialize};

use super::player::SimplePlayer;

#[derive(Serialize, Deserialize)]
pub struct ServerEvents {
    pub xp_multiplier: Option<XPMultiplier>
}

#[derive(Serialize, Deserialize)]
pub struct XPMultiplier {
    pub value: f32,
    pub player: Option<SimplePlayer>,
    pub updated_at: u64
}
