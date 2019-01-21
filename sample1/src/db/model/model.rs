#[derive(Debug, Hash, Default, Serialize, Deserialize)]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub last_name: String,
}
