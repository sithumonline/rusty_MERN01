// extern crate dotenv;
// extern crate mongodb;

use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::doc,
    //bson::{doc, Bson},
    Client,
};
use std::sync::Mutex;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/books").route(web::get().to(get_books)));
}

async fn get_books(data: web::Data<Mutex<Client>>) -> impl Responder {
    let books_collection = data.lock().unwrap().database("T").collection("books");

    //#let filter = doc! {};
    //#   let find_options = FindOptions::builder().sort(doc! { "createdOn": 1 }).build();
    ////let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
    ////let mut cursor = logs_collection.find(filter, find_options).await.unwrap();

    // Query the database for all pets which are cats.
    let mut cursor = books_collection.find(doc! {}, None).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}
