#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod reminder;
use crate::reminder::Reminder;

// #[cfg(test)] mod tests;

#[post("/", data = "<reminder>")]
fn create(reminder: Json<Reminder>) -> Json<Reminder> {
    reminder
}

#[get("/")]
fn read() -> Json<JsonValue> {
    Json(json!([
        "reminder 1",
        "reminder 2"
    ]))
}

#[put("/<id>", data = "<reminder>")]
fn update(id: i32, reminder: Json<Reminder>) -> Json<Reminder> {
    reminder
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({ "status": "OK" }))
}

fn main() {
    rocket::ignite()
        .mount("/reminder", routes![create, update, delete])
        .mount("/reminders", routes![read])
        .launch();
}