use serde::{Deserialize, Serialize};

use super::get_unix_time; // mutual exclusion wrapper

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub message: String,
    pub created_at: f64,
    pub id: usize,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            message: "Hello World from Tauri".to_owned(),
            id: 69420,
            created_at: get_unix_time(),
        }
    }
}
