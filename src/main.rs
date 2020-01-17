//  Author: James DeVore
//  why? to learn rust 

extern crate chrono;

use std::io;
use ::chrono::prelude::*;

fn main() {
    let name = input("What is your name? ").expect("Something failed");

    let age = input("What is your age? ")
    .expect("Not a number or failed")
    .parse::<i32>().expect("Invalid");

    println!("{} is {} years old", &name, &age);

    let current_year = Utc::now().year() ;
    let year_100 = 100 - age + current_year;
    println!("{} will be 100 in the year {}", name, year_100)
}

/// `input` mimics the input function for python
pub fn input(user_message: &str) -> io::Result<String> {
    ///does not give very much help if panics
    use::std::io::Write;


    print!("{}", user_message);
    let mut buffer : String = String::new();
    io::stdout().flush()?;

    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}
