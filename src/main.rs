use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod schema;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&database_url).expect("The connection to the DB failed!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
