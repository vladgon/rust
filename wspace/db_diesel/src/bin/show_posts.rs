// extern crate db_diesel;

use diesel::prelude::*;
use log::debug;

use db_diesel::models::Post;
use db_diesel::util::connection::establish_connection;
use wg_util::common::config::app_config;
use wg_util::common::config::log::LogConfig;
use wg_util::common::config::rust_app::Options;
use wg_util::Result;

fn main() -> Result<()> {
    wg_util::common::config::rust_app::init(Options::LogWithClap(LogConfig::default(), true))?;
    debug!("Got Connection URL {:?}", app_config::settings()?.db.url);
    use db_diesel::schema::posts::dsl::posts;

    let mut connection = establish_connection()?;
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&mut connection)?;

    debug!("Displaying {} posts", results.len());
    for post in results {
        debug!("{:?}, {:?}", post.title, post.body.unwrap_or_default());
    }
    Ok(())
}