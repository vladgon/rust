#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use self::models::{NewPost, Post};

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("mysql://vladg:123@127.0.0.1:3306/webgrid".to_string());
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts::dsl::{id, posts};
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
    }).unwrap()
}