use config::model::*;
pub use config::model::ENV as Env;

//use util;

pub mod model;

pub fn load(env: ENV) -> Result<Config, serde_json::Error> {
    match env {
        ENV::DEV => serde_json::from_str::<Config>(include_str!("app_dev.json")),
        ENV::PROD => serde_json::from_str::<Config>(include_str!("app_prod.json"))
    }
}

#[test]
fn load_config() -> crate::util::ResultOK {
    let map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_str(include_str!("app_dev.json")).unwrap();
//    println!("Map {:?}", map.values().map(|v|v.to));
    assert!(serde_json::from_str::<Config>(include_str!("app_dev.json"))?.app_name.len() > 0);
    Ok(())
}