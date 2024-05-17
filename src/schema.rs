// @generated automatically by Diesel CLI.

diesel::table! {
    roles (name) {
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (user_id, role_name) {
        user_id -> Int4,
        role_name -> Text,
    }
}

diesel::joinable!(users_roles -> roles (role_name));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
    users_roles,
);
