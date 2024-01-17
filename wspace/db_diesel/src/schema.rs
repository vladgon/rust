// @generated automatically by Diesel CLI.

diesel::table! {
    customer (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        lastName -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        published -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customer,
    posts,
);
