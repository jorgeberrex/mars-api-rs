use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagCreateRequest {
    pub name: String,
    pub display: String
}