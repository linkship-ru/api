use std::env;

use actix_web::{App, HttpServer, web::Data};
use mongodb::{Client, Database};
use dotenv::dotenv;

mod models;
mod routes;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_uri: String = env::var("MONGODB_URI").expect("MONGODB_URI not found in your .env");
    let mongodb_database: String = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE not found in your .env");

    let mongo: Client = Client::with_uri_str(mongodb_uri).await.unwrap();
    let db: Database = mongo.database(mongodb_database.as_str());
    let db_data: Data<Database> = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(routes::users::login::handler)
            .service(routes::users::register::handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}