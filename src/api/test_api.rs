use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

use crate::service::test_service;
use actix_web::web::Json;
use serde::Deserialize;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn findFirst() -> impl Responder {
    let post = test_service::findById(1);
    Json(post)
}

pub async fn findById(id: web::Path<i64>) -> impl Responder {
    let post = test_service::findById(id.into_inner());
    Json(post)
}

#[derive(Deserialize, Clone, Debug)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}


pub async fn create(newPost: Json<NewPost>) -> impl Responder {
    println!("{:?}", newPost);
    let post = test_service::create_post(&newPost.title, &newPost.body);
    Json(post)
}

pub async fn deleteById(id: web::Path<i64>) -> impl Responder {
    test_service::deleteById(id.into_inner());
    HttpResponse::Ok().body("success!")
}

pub async fn publishById (id: web::Path<i64>) -> impl Responder{
    let idInner = id.into_inner();
    test_service::publishById(idInner);
    HttpResponse::Ok().body("success!")
}