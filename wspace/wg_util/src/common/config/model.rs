use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Model {
    pub db: DB,
    pub kafka: Kafka,
    pub grpc: Grpc,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct DB {
    pub url: String,
    pub user: String,
    pub password: Option<String>,
    pub noValue: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Kafka {
    pub broker: String,
    pub topic: String,
    pub pollSleep: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Grpc {
    pub host: String,
    pub port: String,
}