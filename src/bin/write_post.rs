extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Post title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];

    println!(
        "Post \"{}\" contents (Press {} when finished):\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
