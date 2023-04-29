use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_u64_time_millis() -> u64 {
    u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()).unwrap_or(u64::MAX)
}
