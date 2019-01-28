table! {
    customer (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        lastName -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        published -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    customer,
    posts,
);
