extern crate db_diesel;
extern crate wg_util;

use std::error::Error;

use diesel::prelude::*;
use tracing::debug;

use db_diesel::models::Post;
use db_diesel::util::connection::establish_connection;
use wg_util::common::config::app_config;

fn main() -> Result<(), Box<dyn Error>> {
    // init();
    wg_util::common::config::rust_app::init()?;
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