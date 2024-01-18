use chrono::Local;

#[derive(serde::Serialize)]
pub struct SampleData {
    pub name: String,
    pub time: chrono::DateTime<Local>,
}
