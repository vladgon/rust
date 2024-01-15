#[derive(serde::Deserialize, Clone, Debug)]
pub struct Model {
    pub db: DB,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct DB {
    pub url: String,
    pub user: String,
    pub password: Option<String>,
    pub noValue: Option<String>,
}