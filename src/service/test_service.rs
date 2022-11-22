use diesel::prelude::*;
use crate::models::models::{Post, NewPost};
use crate::database::schema::posts;

use crate::database::mysql::establish_connection;


pub fn create_post(title: &str, body: &str) -> Post {
    let connection = &mut establish_connection();

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(connection).unwrap()
}

pub fn findById(id: i64) -> Post {
    let connection = &mut establish_connection();
    posts::dsl::posts.find(id).first(connection).unwrap()
    // posts::table.find(id).first(connection).unwrap();
}

pub fn deleteById(id: i64) -> bool {
    let connection = &mut establish_connection();

    diesel::delete(posts::table).filter(posts::id.eq(id))
        .execute(connection).unwrap();

    return true;
}