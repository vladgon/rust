extern crate sample1;

use sample1::config;

//use sample1::util;

#[test]
fn test_config() {
    assert!(config::load(config::Env::DEV)
        .map(|res| res.app_name.len() > 0)
        .expect("Should have value"));
}
