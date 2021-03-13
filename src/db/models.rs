use super::schema::{members, lists, list_member_relation};

#[derive(Queryable, Identifiable)]
pub struct Member {
    pub id: i32,
    pub email: String,
    pub hash: Option<String>,
    pub enabled: bool,
    pub token: Option<String>,
}

#[derive(Default, AsChangeset)]
#[table_name="members"]
pub struct MemberUpdate<'a> {
    pub email: Option<&'a str>,
    pub hash: Option<&'a str>,
    pub enabled: Option<bool>,
}

#[derive(Insertable)]
#[table_name="members"]
pub struct NewMember<'a> {
    pub email: &'a str,
    pub hash: Option<&'a str>,
}

// Lists

#[derive(Queryable, Identifiable, AsChangeset)]
#[table_name="lists"]
pub struct List {
    pub id: i32,
    pub name: String,
    pub mail_identifier: String,
}

#[derive(Insertable)]
#[table_name="lists"]
pub struct NewList<'a> {
    pub name: &'a str,
    pub mail_identifier: &'a str
}

//Relations

#[derive(Queryable, Associations, Identifiable)]
#[table_name="list_member_relation"]
#[belongs_to(List, foreign_key="list")]
#[belongs_to(Member, foreign_key="member")]
pub struct ListMembership {
    pub id: i32,
    pub list: i32,
    pub member: i32,
}