extern crate diesel;
extern crate cakebot_api;

use self::cakebot_api::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Enter name: ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's create a reminder for {} (Press {} when finished)\n", name, EOF);
    let mut date = String::new();
    stdin().read_to_string(&mut date).unwrap();

    let post = create_post(&connection, name, &date);
    println!("\nSaved reminder for {} with id {}", name, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";