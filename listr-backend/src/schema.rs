table! {
    items (id) {
        id -> Int4,
        list_id -> Int4,
        data -> Varchar,
    }
}

table! {
    lists (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(items -> lists (list_id));

allow_tables_to_appear_in_same_query!(
    items,
    lists,
);
