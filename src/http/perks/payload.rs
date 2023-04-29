use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinSoundSetRequest {
    pub active_join_sound_id: Option<String>
}