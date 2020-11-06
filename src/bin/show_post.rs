extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;
    
    let connection = establish_connection();
    let result = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", result.len());
    for post in result {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
