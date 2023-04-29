use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankCreateRequest {
    pub name: String,
    pub display_name: Option<String>,
    pub priority: u32,
    pub prefix: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub staff: bool,
    #[serde(default)]
    pub apply_on_join: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankUpdateRequest {
    pub name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub priority: u32,
    #[serde(default)]
    pub prefix: Option<String>,
    pub permissions: Vec<String>,
    pub staff: bool,
    pub apply_on_join: bool
}
