use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleData {
    pub name: String,
    pub time: chrono::DateTime<Local>,
}
