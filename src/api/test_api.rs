use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

use crate::service::test_service;
use actix_web::web::Json;
use serde::Deserialize;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn find_first() -> impl Responder {
    let post = test_service::find_by_id(1);
    Json(post)
}

pub async fn find_by_id(id: web::Path<i64>) -> impl Responder {
    let post = test_service::find_by_id(id.into_inner());
    Json(post)
}

#[derive(Deserialize, Clone, Debug)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}


pub async fn create(new_post: Json<NewPost>) -> impl Responder {
    println!("{:?}", new_post);
    let post = test_service::create_post(&new_post.title, &new_post.body);
    Json(post)
}

pub async fn delete_by_id(id: web::Path<i64>) -> impl Responder {
    test_service::delete_by_id(id.into_inner());
    HttpResponse::Ok().body("success!")
}

pub async fn publish_by_id (id: web::Path<i64>) -> impl Responder{
    let id_inner = id.into_inner();
    test_service::publish_by_id(id_inner);
    HttpResponse::Ok().body("success!")
}