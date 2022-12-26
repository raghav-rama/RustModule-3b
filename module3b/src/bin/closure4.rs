// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

use std::io;

fn main() {
    println!("Enter name: ");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("problem reading stdin");
    buf = buf.trim().to_string();
    let res: Option<User> = find_user(buf.as_str()).map(|user_id| User { user_id, name: buf });
    match res {
        Some(data) => println!("{:?}", data),
        None => println!("Not found"),
    }
}

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "trizwit" => Some(1),
        "prathamesh" => Some(3),
        "nagi" => Some(7),
        _ => None,
    }
}
