use super::schema::members;
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