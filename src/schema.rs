// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        answer -> Varchar,
        is_correct -> Bool,
        question_id -> Int4,
        created_at -> Timestamp,
    }
}

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
    classes (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        description -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    classes_students (class_id, student_id) {
        class_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    exam_questions (exam_id, question_id) {
        exam_id -> Int4,
        question_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exams (id) {
        id -> Int4,
        name -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        class_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question -> Varchar,
        created_at -> Timestamp,
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

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(avatars -> users (user_id));
diesel::joinable!(classes -> users (user_id));
diesel::joinable!(classes_students -> classes (class_id));
diesel::joinable!(classes_students -> users (student_id));
diesel::joinable!(exam_questions -> exams (exam_id));
diesel::joinable!(exam_questions -> questions (question_id));
diesel::joinable!(exams -> classes (class_id));
diesel::joinable!(users_roles -> roles (role_name));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    avatars,
    classes,
    classes_students,
    exam_questions,
    exams,
    questions,
    roles,
    users,
    users_roles,
);
