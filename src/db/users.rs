use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use super::models::*;

pub fn display_all(conn : &MysqlConnection) {
    use super::schema::members::dsl::*;

    let res = members
        .load::<Member>(conn)
        .expect("Failed to find members");

    for mem in res {
        println!("{}", mem.email);
    }
}

pub fn create_user<'a>(conn: &MysqlConnection, email: &'a str, hash: Option<&'a str>) {
    use super::schema::members;

    let new_member = NewMember{
        email: email,
        hash: hash
    };

    diesel::insert_into(members::table)
    .values(&new_member)
    .execute(conn)
    .expect("Failed to save member");
}