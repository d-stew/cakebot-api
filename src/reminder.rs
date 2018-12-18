#[derive(Serialize, Deserialize)]

pub struct Reminder {
  pub id: Option<i32>,
  pub first_name: String,
  pub last_name: Option<String>,
  pub date: String,
  pub buffer: Option<i32>,
}