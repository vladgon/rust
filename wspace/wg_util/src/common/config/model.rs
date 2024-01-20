use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Model {
    pub db: DB,
    pub kafka: Kafka,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DB {
    pub url: String,
    pub user: String,
    pub password: Option<String>,
    pub noValue: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Kafka {
    pub broker: String,
    pub topic: String,
    pub pollSleep: u64,
}