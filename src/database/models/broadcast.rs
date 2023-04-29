use serde::{Serialize, Deserialize};

fn default_newline() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Broadcast {
    pub name: String,
    pub message: String,
    pub permission: Option<String>,
    #[serde(default = "default_newline")]
    pub newline: bool
}
