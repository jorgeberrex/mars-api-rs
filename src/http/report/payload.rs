use serde::{Serialize, Deserialize};

use crate::database::models::player::SimplePlayer;

#[derive(Serialize, Deserialize)]
pub struct ReportCreateRequest {
    pub target: SimplePlayer,
    pub reporter: SimplePlayer,
    pub reason: String,
    #[serde(rename = "onlineStaff")]
    pub online_staff: Vec<SimplePlayer>,
}
