#[cfg(test)]
mod test {
    #[test]
    fn env() {
        assert!(dotenv::var("DB_URL").is_ok());
        if let Ok(lang) = dotenv::var("DB_URL") {
            println!("DB_URL: {}", lang);
        } else {
            println!("Couldn't read DB_URL");
        }
        match dotenv::var("DB_URL") {
            Ok(lang) => println!("DB_URL: {}", lang),
            Err(e) => println!("Couldn't read DB_URL ({:?})", e),
        }
    }

    #[test]
    fn include() {
//    let res = std::include!("config.txt");
        let map: std::collections::HashMap<_, _> = [
            ("Norway", 100),
            ("Denmark", 50),
            ("Iceland", 10)].iter().cloned().collect();


        println!("config {:?}", map);
    }
}