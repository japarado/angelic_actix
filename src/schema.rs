table! {
    comments (id) {
        id -> Int4,
        body -> Varchar,
        post_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        hash -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
