use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};

pub type AppPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create() -> ConnectionManager<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    manager
}
