//  Author: James DeVore
//  why? to learn rust 

use std::io;

fn main() {
    let name = input("What is your name? ").expect("Something failed");

    let age = input("What is your age? ")
    .expect("Not a number or failed")
    .parse::<u8>().expect("Invalid");

    println!("{} is {} years old", name, age);
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
