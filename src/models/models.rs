use diesel::prelude::*;
use crate::database::schema::*;
use serde::{Deserialize,Serialize};

#[derive(Queryable,Serialize,Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable,Serialize,Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
