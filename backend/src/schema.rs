// @generated automatically by Diesel CLI.

diesel::table! {
    booking (id) {
        id -> Nullable<Integer>,
        reason -> Text,
        duration -> Text,
        is_approved -> Nullable<Bool>,
        fk_room_id -> Integer,
        fk_user_id -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    room (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        is_admin -> Nullable<Bool>,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(booking -> room (fk_room_id));
diesel::joinable!(booking -> user (fk_user_id));

diesel::allow_tables_to_appear_in_same_query!(booking, room, user,);
