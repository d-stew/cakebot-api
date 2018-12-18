extern crate diesel;
extern crate cakebot_api;

use self::cakebot_api::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
  use self::schema::reminders::dsl::*;

    let connection = establish_connection();
    let results = reminders
        .limit(5)
        .load::<Reminder>(&connection)
        .expect("Error loading reminders");

    println!("Displaying {} reminders", results.len());
    for reminder in reminders {
        println!("{}", reminder.remind_date);
        println!("----------\n");
        println!("{}", reminder.name);
        println!("----------\n");
        println!("{}", reminder.buffer);
    }
}