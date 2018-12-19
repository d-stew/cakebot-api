#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

use self::models::{Reminder, NewReminder};

pub fn create_reminder<'a>(conn: &PgConnection, name: &'a str, remind_date: &'a str) -> Reminder {
    use schema::reminders;

    let new_reminder = NewReminder {
        name: name,
        remind_date: remind_date,
    };

    diesel::insert_into(reminders::table)
        .values(&new_reminder)
        .get_result(conn)
        .expect("Error saving new reminder")
}