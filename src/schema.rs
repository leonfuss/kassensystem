table! {
    transactions (id) {
        id -> Integer,
        description -> Text,
        date -> Timestamp,
        authorization -> Integer,
        affected -> Integer,
        amount -> Float,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        date_created -> Timestamp,
        balance -> Float,
    }
}

allow_tables_to_appear_in_same_query!(
    transactions,
    users,
);
