table! {
    reminders (id) {
        id -> Int4,
        name -> Varchar,
        remind_date -> Date,
        note -> Nullable<Text>,
        buffer -> Nullable<Int4>,
    }
}
