#![feature(try_trait)]
extern crate db_diesel;

use std::error::Error;

use diesel::prelude::*;
use tracing::debug;

use db_diesel::config::init;

use crate::util::connection::establish_connection;

use self::db_diesel::*;
use self::models::*;

fn main() -> Result<(), Box<dyn Error>> {
    init();
    debug!("Got Connection URL {:?}", config::db_url());
    use db_diesel::schema::posts::dsl::posts;

    let connection = establish_connection()?;
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)?;

    debug!("Displaying {} posts", results.len());
    for post in results {
        debug!("{:?}, {:?}", post.title, post.body.unwrap_or_default());
    }
    Ok(())
}