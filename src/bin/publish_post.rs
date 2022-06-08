extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("Post ID is required")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Post \"{}\" is published successfully.", post.title);
}