use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestroyableDamageData {
    pub destroyable_id: String,
    pub damage: u32,
    pub player_id: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalContribution {
    pub player_id: String,
    pub percentage: f32,
    pub block_count: u32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestroyableDestroyData {
    pub destroyable_id: String,
    pub contributions: Vec<GoalContribution>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreLeakData {
    pub core_id: String,
    pub contributions: Vec<GoalContribution>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlPointCaptureData {
    pub point_id: String,
    pub player_ids: Vec<String>,
    pub party_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagDropData {
    pub flag_id: String,
    pub player_id: String,
    pub held_time: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagEventData {
    pub flag_id: String,
    pub player_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoolEventData {
    pub wool_id: String,
    pub player_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoolDropData {
    pub wool_id: String,
    pub player_id: String,
    pub held_time: u64,
}
