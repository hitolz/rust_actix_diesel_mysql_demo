use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing ::{
    event,Level
};
use tracing::instrument::WithSubscriber;
use time::{macros::format_description, UtcOffset};
use actix_web::web::Json;

mod api;
mod service;
mod database;
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    let post = service::test_service::create_post("test--tile", "hhhhhhh");
    Json(post)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_timer(local_time)
        .init();

    let ip = "127.0.0.1";
    let port = 8099;

    let mut address = String::new();
    address.push_str(ip);
    address.push_str(":");
    address.push_str(&*port.to_string());
    tracing::info!("server start http://{}",address);


    HttpServer::new( || {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(api::test_api::hello))
    })
        .bind((ip,port))?
        .run()
        .await
}