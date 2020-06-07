extern crate db_diesel as db;


#[cfg(test)]
mod test {
    use std::error::Error;

    use db::config;
    use db::create_post;
    use db::util::connection::establish_connection;

    #[test]
    fn test_config() {
        // let cd = &env::current_dir().unwrap();
        config::init();
        tracing::debug!("Test 1 {:?}", config::db_url());
        tracing::debug!("Test 2 {:?}", config::db_url());
    }

    #[test]
    #[ignore]
    fn db_integration() {
        config::init();
        tracing::debug!("{:?}",config::db_url());
    }

    #[test]
    #[ignore]
    fn insert_post() -> Result<(), Box<dyn Error>> {
        config::init();
        tracing::debug!("Inserted {:?}", create_post(&establish_connection()?, "vlad", "Post body"));
        Ok(())
    }
}
