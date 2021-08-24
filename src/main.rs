use std::env;
mod hangman;
mod gtin1;
mod gtin2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args.get(1).expect("Please enter a problem");

    match problem.as_ref() {
        "hangman" => hangman::main(),
        "gtin1" => gtin1::main(),
        "gtin2" => gtin2::main(),
        _ => println!("Unknown problem '{}'", problem),
    }
}