use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod services;
mod models;
mod database;
mod views;
mod controllers;

use controllers::vaults_controller::upsert_vault_handler;

pub mod schema;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/vaults/{vault_id}", web::put().to(upsert_vault_handler))
            .route("/vaults", web::put().to(upsert_vault_handler)) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
