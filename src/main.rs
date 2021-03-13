extern crate futures;
extern crate mail_core;
extern crate mail_smtp;
#[macro_use] extern crate mail_headers;
#[macro_use] extern crate diesel;
extern crate dotenv;

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
}