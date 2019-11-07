table! {
    urls (id) {
        id -> Int4,
        url -> Varchar,
        active -> Bool,
    }
}

table! {
    url_status (id) {
        id -> Int4,
        url_id -> Int4,
        created_at -> Timestamp,
        http_status -> Int4,
    }
}

joinable!(url_status -> urls (url_id));

allow_tables_to_appear_in_same_query!(
    urls,
    url_status,
);
