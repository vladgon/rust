#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate util;

use std::env;

use diesel::prelude::*;

use self::models::{NewPost, Post};

pub mod schema;
pub mod models;

///
/// Establish Connection
///
/// # Examples
/// ```
/// use diesel::connection::SimpleConnection;
/// assert!(db_diesel::establish_connection().batch_execute("Select 1").is_ok(), "Cannot get Connection")
/// ```
///
pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("mysql://vladg:123@127.0.0.1:3306/webgrid".to_string());
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Result<Post, diesel::result::Error> {
    use schema::posts::dsl::{posts, id};
    let new_post = NewPost {
        title,
        body,
    };

    conn.transaction(|| {
        diesel::insert_into(posts)
            .values(&new_post)
            .execute(conn)
            .expect("Error saving new post");

        posts.order(id.desc()).first(conn)
    })
}