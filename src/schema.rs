// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        acc_password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        creation_date -> Nullable<Date>,
    }
}


diesel::table! {
    sessions (user_id, session_id) {
        user_id -> Int4,
        session_id -> Varchar
    }

}

diesel::allow_tables_to_appear_in_same_query!(
    users, sessions,
);

diesel::joinable!(
    sessions -> users (user_id)
);