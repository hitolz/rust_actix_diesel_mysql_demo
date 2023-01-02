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

pub fn find_by_id(id: i64) -> Post {
    let connection = &mut establish_connection();
    // 几种写法都能运行、、
    // posts::table.filter(posts::id.eq(id)).first(connection).unwrap()
    // posts::dsl::posts.find(id).first(connection).unwrap()
    posts::table.find(id).first(connection).unwrap()
}

pub fn delete_by_id(id: i64) -> bool {
    let connection = &mut establish_connection();

    diesel::delete(posts::table).filter(posts::id.eq(id))
        .execute(connection).unwrap();

    return true;
}

pub fn publish_by_id(id: i64) -> bool {
    let connection = &mut establish_connection();

    diesel::update(posts::table).filter(posts::id.eq(id))
        .set(posts::published.eq(true))
        .execute(connection).unwrap();
    true
}