#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewPost, Post};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) {
    use schema::posts;
    
    let new_post = NewPost {
        title: title,
        body: body,
        published: &0,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
    
    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(conn)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}

