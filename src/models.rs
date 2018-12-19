use super::schema::reminders;

#[derive(Queryable)]
pub struct Reminder {
  pub id: Option<i32>,
  pub name: String,
  pub remind_date: String,
  pub note: Option<String>,
  pub buffer: Option<i32>,
}

#[derive(Insertable)]
#[table_name="reminders"]
pub struct NewReminder<'a> {
    pub name: &'a str,
    pub remind_date: &'a str,
    pub note: &'a str,
}