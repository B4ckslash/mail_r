use super::schema::members;
#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub email: String,
    pub hash: Option<String>,
    pub enabled: Option<bool>,
    pub token: Option<String>,
}

#[derive(Insertable)]
#[table_name="members"]
pub struct NewMember<'a> {
    pub email: &'a str,
    pub hash: Option<&'a str>,
}