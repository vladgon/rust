extern crate dotenv;

use std::env;

#[test]
fn env() {
    match dotenv::var("DB_URL") {
        Ok(lang) => println!("DB_URL: {}", lang),
        Err(e) => println!("Couldn't read DB_URL ({:?})", e),
    };
}

#[test]
fn include() {
//    let res = std::include!("config.txt");
    let map: std::collections::HashMap<_, _> = [("Norway", 100),
        ("Denmark", 50),
        ("Iceland", 10)].iter().cloned().collect();


    println!("Config {:?}", map);
}