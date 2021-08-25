#[macro_use]
extern crate rocket;

use std::env;
mod gtin1;
mod gtin2;
mod gtin3;
mod hangman;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args.get(1).expect("Please enter a problem");

    match problem.as_ref() {
        "hangman" => hangman::main(),
        "gtin1" => gtin1::main(),
        "gtin2" => gtin2::main(),
        "gtin3" => gtin3::main(),
        "server" => server::main(),
        _ => println!("Unknown problem '{}'", problem),
    }
}
