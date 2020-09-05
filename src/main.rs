extern crate dotenv;

use dotenv::dotenv;
use actix_web::get;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use mongodb::{options::ClientOptions, Client};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}
#[get("/again")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    dotenv::dotenv().expect("Failed to read .env file");
    let case_sensitive = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    println!("Env : {}", case_sensitive);
    
    let mut server = HttpServer::new(|| App::new().service(index).service(index2).service(index3));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
