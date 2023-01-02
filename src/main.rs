use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing ::{
    event,Level
};
use tracing::instrument::WithSubscriber;
use time::{macros::format_description, UtcOffset};
use actix_web::web::Json;
use api::test_api;

mod api;
mod service;
mod database;
mod models;
mod settings;

#[get("/")]
async fn hello() -> impl Responder {
    let post = service::test_service::create_post("test--title", "hhhhhhh");
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
            .route("/hey", web::get().to(test_api::hello))
            .route("/create",web::post().to(test_api::create))
            .route("/findFirst",web::get().to(test_api::find_first))
            .route("/findById/{id}",web::get().to(test_api::find_by_id))
            .route("/deleteById/{id}",web::post().to(test_api::delete_by_id))
            .route("/publish/{id}",web::post().to(test_api::publish_by_id))

    })
        .bind((ip,port))?
        .run()
        .await
}