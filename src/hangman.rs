use std::io;
use std::io::Write;
use std::fs::read_to_string;
use rand::Rng;

pub fn main() {
    let words = read_to_string("words.txt").unwrap();
    let words: Vec<String> = words.lines().map(|s| s.to_uppercase()).collect();
    let word: Vec<char> = words[rand::thread_rng().gen_range(0..words.len())].chars().collect();
    let mut guessed: Vec<Option<char>> = vec![None; word.len()];
    let mut lives = word.len();
    while lives > 0 {
        for ltr in guessed.iter() {
            print!("{} ", if let Some(chr) = ltr {*chr} else {'_'});
        }
        if !guessed.contains(&None) {
            println!("\nYou win!");
            break
        }

        print!("\nGuess a letter > ");
        io::stdout().flush().unwrap();
        let mut rawguess = String::new();
        io::stdin().read_line(&mut rawguess).unwrap();
        rawguess = rawguess.trim().to_uppercase();
        let guess = rawguess.chars().next();
        if let None = guess {
            println!("Please enter a letter!");
            continue
        }
        let guess = guess.unwrap();

        if word.contains(&guess) {
            for (ind, ltr) in word.iter().enumerate() {
                if ltr == &guess {
                    guessed[ind] = Some(*ltr);
                }
            }
        } else {
            lives -= 1;
            println!("That's not in the word! You have {} {} remaining.", lives, if lives != 1 {"lives"} else {"life"})
        }
    }
    if lives == 0 {
        println!("You lose. The word was {}.", word.into_iter().collect::<String>())
    }
}