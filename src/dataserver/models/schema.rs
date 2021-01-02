table! {
    users (id) {
        id -> Int8,
        email -> Text,
        signup_source -> Text,
        access_token -> Text,
        token_expire_time -> Timestamp,
        profile_picture -> Nullable<Text>,
        first_name -> Text,
        last_name -> Text,
        phone -> Nullable<Text>,
        location -> Nullable<Text>,
        last_logged_in_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
