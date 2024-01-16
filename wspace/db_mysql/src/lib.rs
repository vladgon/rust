use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Conf {
    db: Db,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Db {
    url: Url,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Url {
    name: String,
    path: String,
}

#[cfg(test)]
mod test {
    use config::{Config, File};

    use crate::Conf;

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
    fn conf() {
        let conf = Config::builder()
            .add_source(File::with_name("./tests.yaml"))
            .build()
            .map(Config::try_deserialize::<Conf>)
            .ok()
            .unwrap();

        println!("Config  - {:?}", conf);
        println!("Name  - {:?}", conf.unwrap().db.url.name);
    }

    #[test]
    fn include() {
//    let res = std::include!("wg_sample_app.txt");
        let map: std::collections::HashMap<_, _> = [
            ("Norway", 100),
            ("Denmark", 50),
            ("Iceland", 10)].iter().cloned().collect();


        println!("wg_sample_app {:?}", map);
    }
}