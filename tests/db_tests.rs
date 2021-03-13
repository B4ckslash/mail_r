extern crate diesel;

mod common;

use core::panic;
use diesel::Connection;
use diesel::result::Error;
use diesel::mysql::MysqlConnection;
use mailr::db;

#[test]
fn test_users() {
    let conn = common::get_connection();
    conn.test_transaction::<_, Error, _>(|| {
        let email = "test@example.com";
        test_user_creation(&conn, email);
        test_user_update(&conn, email);
        test_user_deletion(&conn, email);
        Ok(())
    });
}

#[test]
#[should_panic]
fn test_user_update_fail() {
    let conn = common::get_connection();
    conn.test_transaction::<_, Error, _>(|| {
        let email = "test@example.com";
        let wrong_email = "tset@example.com";
        test_user_creation(&conn, email);
        test_user_update(&conn, wrong_email);
        Ok(())
    });
}

#[test]
#[should_panic]
fn test_user_deletion_fail() {
    let conn = common::get_connection();
    conn.test_transaction::<_, Error, _>(|| {
        let email = "test@example.com";
        let wrong_email = "tset@example.com";
        test_user_creation(&conn, email);
        test_user_deletion(&conn, wrong_email);
        Ok(())
    });
}

fn test_user_creation(conn: &MysqlConnection, email: &str) {
    let email = "test@example.com";
    let user = db::users::create_user(conn, email, None);
    assert!(db::users::get_user_by_email(conn, email).is_some());
}

fn test_user_update(conn: &MysqlConnection, email: &str) {
    let test_user = &db::users::get_user_by_email(conn, email);
    db::users::update_user(
        conn,
        test_user,
        &db::models::MemberUpdate {
            hash: Some("156"),
            ..Default::default()
        },
    );
    match db::users::get_user_by_email(conn, email) {
        Some(member) => assert_eq!(member.hash.expect("The hash should not be empty!"), "156"),
        None => panic!(),
    }
}

fn test_user_deletion(conn: &MysqlConnection, email: &str) {
    let test_user = &db::users::get_user_by_email(conn, email);
    match db::users::delete_user(conn, test_user) {
        Ok(changed) => assert_eq!(1, changed),
        Err(_) => panic!(),
    }
}
