//! ORM types generated by Diesel.
//!
//! Each child module represents a table in the database.

#![allow(missing_docs)]

table! {
    access_tokens {
        id -> BigSerial,
        user_id -> Text,
        value -> Text,
        revoked -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    events {
        id -> Text,
        ordering -> BigSerial,
        room_id -> Text,
        user_id -> Text,
        event_type -> Text,
        state_key -> Nullable<Text>,
        content -> Text,
        extra_content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    room_aliases (alias) {
        alias -> Text,
        room_id -> Text,
        servers -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rooms {
        id -> Text,
        user_id -> Text,
        public -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users {
        id -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
