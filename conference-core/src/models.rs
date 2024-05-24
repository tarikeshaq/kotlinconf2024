use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub content: String,
    pub id: u64,
    pub time_created: u128,
}
