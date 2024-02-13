mod controllers;
pub mod repository;
mod database;

use diesel::{r2d2};
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let manager = database::connection::create();

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    web::HttpServer::new(move || {
        web::App::new().state(pool.clone())
            .service(controllers::user::get)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

