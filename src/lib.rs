#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use crate::models::{Post, NewPost};
use diesel::pg::PgConnection;
// dotenvは.env内の環境変数を取得することができる
use dotenv:: dotenv;
use std::env;


pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost{
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
