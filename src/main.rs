// TODO : Create a palindrome game in rust breakit and make a new one
//TODO: : New one equates if it's a pal or not 😁

mod check;
use std::io::{self, Write};

use check::{is_case_palindrome, is_not_case_palindrome};

fn main() {
    // Prompt the user for input

    print!("Enter a string:");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read_line");
    // Trim the whole to remove whitespaces
    let user_input = user_input.trim();

    // Prompt the user for case-sensitive
    print!("check case-sensitive (y/n)? ");
    io::stdout().flush().unwrap();

    let mut case_sensitive = String::new();
    io::stdin().read_line(&mut case_sensitive).expect("Fail to read_line");
    let case_sensitive = case_sensitive.trim().to_lowercase();


    // check if  string is a case-sensitive palindrome

    if case_sensitive == "y"{
        if 
    } 
}
