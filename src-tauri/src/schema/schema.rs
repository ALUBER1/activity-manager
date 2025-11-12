diesel::table! {
    events(uuid) {
        uuid -> Text,
        name -> Text,
        date -> Text,
        time -> Text,
        notified_at -> Text
    }
}