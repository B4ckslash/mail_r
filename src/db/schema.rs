table! {
    list_member_relation (id) {
        id -> Integer,
        list -> Integer,
        member -> Integer,
    }
}

table! {
    lists (id) {
        id -> Integer,
        name -> Varchar,
        mail_identifier -> Varchar,
    }
}

table! {
    members (id) {
        id -> Integer,
        email -> Varchar,
        hash -> Nullable<Char>,
        enabled -> Bool,
        ephem_token -> Nullable<Char>,
    }
}

joinable!(list_member_relation -> lists (list));
joinable!(list_member_relation -> members (member));

allow_tables_to_appear_in_same_query!(
    list_member_relation,
    lists,
    members,
);
