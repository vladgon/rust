#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::result::Error;

use wg_util::Result;

use self::models::{NewPost, Post};

pub mod schema;
pub mod models;
pub mod util;
pub mod config;

pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str) -> Result<Post> {
    use crate::schema::posts::dsl::{id, posts};
    let new_post = NewPost {
        title,
        body,
    };

    conn.transaction(|conn| {
        diesel::insert_into(posts)
            .values(&new_post)
            .execute(conn)
            .expect("Error saving new post");

        posts.order(id.desc()).first(conn)
    })
        .map_err(Error::into)
}