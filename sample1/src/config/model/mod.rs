#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub app_name: String,
    pub db_user: String,
    pub db_pwd: String,
    pub dp_prop: DbProp,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DbProp {
    db_name: String
}

#[derive(Debug)]
pub enum ENV {
    DEV,
    PROD,
}