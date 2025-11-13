diesel::table! {
    events(uuid) {
        uuid -> Text,
        name -> Text,
        date -> Date,
        time -> Time,
        notified_at -> Text
    }
}
