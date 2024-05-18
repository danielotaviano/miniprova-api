// @generated automatically by Diesel CLI.

diesel::table! {
    avatars (id) {
        id -> Int4,
        user_id -> Int4,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (user_id, role_name) {
        user_id -> Int4,
        role_name -> Text,
    }
}

diesel::joinable!(avatars -> users (user_id));
diesel::joinable!(users_roles -> roles (role_name));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(avatars, roles, users, users_roles,);
