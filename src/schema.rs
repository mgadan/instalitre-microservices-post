table! {
    posts (id) {
        id -> Uuid,
        author -> Uuid,
        description -> Text,
        photo -> Uuid,
        created_at -> Timestamp,
    }
}
