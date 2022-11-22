use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};


pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}