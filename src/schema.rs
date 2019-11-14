table! {
    posts (id) {
        id -> Uuid,
        author -> Uuid,
        description -> Text,
        photo -> Text,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        price -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    products,
);
