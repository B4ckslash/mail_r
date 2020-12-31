extern crate futures;
extern crate mail_core;
extern crate mail_smtp;
#[macro_use] extern crate mail_headers;
#[macro_use] extern crate diesel;
extern crate dotenv;

mod db;

use futures::Future;
use mail_headers::{
    headers::*,
    header_components::Domain
};

use mail_core::{Mail, default_impl::simple_context};
use mail_smtp::{self as smtp, ConnectionConfig};

fn main()
{
    let ctx = simple_context::new(Domain::from_unchecked("example.com".to_owned()), "mailer".parse().unwrap()).unwrap();
    let conn = &db::establish_connection();
    db::display_users(conn);
    db::users::create_user(conn, "test@example.com", None);
    db::display_users(conn);
    let test_user = &db::users::get_user_by_email(conn, "test@example.com");
    db::users::update_user(conn, test_user, &db::models::MemberUpdate {hash: Some("156"), ..Default::default()});
    db::display_users(conn);
    db::users::delete_user(conn, test_user);
    db::display_users(conn);
}