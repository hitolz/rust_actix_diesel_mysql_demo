use diesel::prelude::*;
use crate::models::models::{Post, NewPost};
use crate::database::schema::posts;

use crate::database::mysql::establish_connection;

pub fn create_post( title: &str, body: &str) -> Post {

    let connection = &mut establish_connection();
    use crate::database::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(connection).unwrap()
}
