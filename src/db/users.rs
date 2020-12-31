use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use super::models::*;

pub fn display_all(conn : &MysqlConnection) {
    use super::schema::members::dsl::*;

    let res = members
        .load::<Member>(conn)
        .expect("Failed to find members");

    for mem in res {
        println!("Mail: {}; Hash: {}; Enabled: {}", mem.email, mem.hash.unwrap_or("<empty>".to_string()), mem.enabled);
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

pub fn update_user(conn: &MysqlConnection, user: &Option<Member>, data: &MemberUpdate)
{
    match user {
        Some(member) => diesel::update(member).set(data).execute(conn)
                                      .expect(format!("Failed to update member with id {}", member.id).as_ref()),
        None => 0,
    };
}

pub fn delete_user(conn: &MysqlConnection, user: &Option<Member>) {
    match user{
        Some(member) => diesel::delete(member)
                                .execute(conn)
                                .expect(format!("Failed to delete user with ID {}", member.id).as_ref()),
        None => 0,
    };
}

pub fn get_user_by_email<'a>(conn: &MysqlConnection, expected_email: &'a str) -> Option<Member> {
    use super::schema::members::dsl::*;

    let mut found = members.filter(email.eq(expected_email))
    .limit(1)
    .load::<Member>(conn)
    .expect(format!("Failed to find user with email {}", expected_email).as_ref());

    if found.is_empty()
    {
        return None;
    }

    Some(found.remove(0))
}