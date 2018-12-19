#[derive(Serialize, Deserialize)]

pub struct Reminder {
  pub id: Option<i32>,
  pub name: String,
  pub remind_date: String,
  pub note: Option<String>,
  pub buffer: Option<i32>,
}