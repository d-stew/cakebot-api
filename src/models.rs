#[derive(Queryable)]
pub struct Reminder {
  pub id: Option<i32>,
  pub name: String,
  pub remind_date: String,
  pub note: String,
  pub buffer: Option<i32>,
}