pub mod users;
pub mod lists;
pub mod schema;

pub mod models;

#[macro_use]
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
    .expect("Unset envvar DATABASE_URL");

    MysqlConnection::establish(&db_url)
    .expect(&format!("Failed to connect to database with URL {}", db_url))
}

pub fn display_users(conn: &MysqlConnection) {
    users::display_all(conn)
}