use actix_web::{App, HttpServer, web::Data};
use mongodb::{Client, Database};

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo: Client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db: Database = mongo.database("linkship");
    let db_data: Data<Database> = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(routes::users::login::handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}