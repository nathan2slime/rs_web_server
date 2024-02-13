use ntex::web::{self, Error, HttpResponse};
use serde::Serialize;
use crate::database::connection::AppPool;
use crate::repository;

#[derive(Serialize)]
struct User {
    username: String,
    age: i32,
}

#[web::get("/user")]
pub async fn get(pool: web::types::State<AppPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let users = web::block(move || repository::user::get_users(&mut conn)).await?;

    Ok(HttpResponse::Ok().json(&users))
}