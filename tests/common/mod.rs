extern crate dotenv;

use diesel::Connection;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn get_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
    .expect("Unset envvar DATABASE_URL");

    MysqlConnection::establish(&db_url)
    .expect(&format!("Failed to connect to database with URL {}", db_url))
}