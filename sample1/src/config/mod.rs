use config::model::config_model::*; 
pub use config::model::config_model::ENV as Env;

pub mod model;

pub fn load(env: ENV) -> Result<Config, serde_json::Error> {
    return match env {
        ENV::DEV => serde_json::from_str::<Config>(include_str!("app_dev.json")),
        ENV::PROD => serde_json::from_str::<Config>(include_str!("app_prod.json"))
    };
}
