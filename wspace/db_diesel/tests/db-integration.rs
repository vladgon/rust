extern crate db_diesel as db;


#[cfg(test)]
mod test {
    use ctor::ctor;
    use log::debug;

    use db::create_post;
    use db::util::connection::establish_connection;
    use wg_util::common::config::{app_config, rust_app};
    use wg_util::common::config::rust_app::Options;
    use wg_util::Result;

    #[ctor]
    fn init() {
        rust_app::init(Options::DefaultLog(false)).unwrap();
    }

    #[test]
    fn test_config() {
        debug!("Test 1 {:?}", app_config::settings().map(|s|&s.db));
    }


    #[test]
    fn insert_post() -> Result<()> {
        debug!("Inserted {:?}", create_post(&mut establish_connection()?, "vlad", "Post body"));
        Ok(())
    }
}
