#[macro_use]
extern crate diesel;

use std::sync::Once;

use diesel::prelude::*;

use self::models::{NewPost, Post};

pub mod schema;
pub mod models;
pub mod util;
pub mod config;


static START: Once = Once::new();


pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts::dsl::{posts, id};
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