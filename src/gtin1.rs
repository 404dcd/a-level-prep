use std::io;
use std::io::Write;

fn get_n_ints(n: i32) -> Result<Vec<i32>, &'static str> {
    print!("Enter {} digit number, including any leading zeros > ", n);
    io::stdout().flush().unwrap();
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let ints: Vec<i32> = number
        .trim()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as i32)
        .collect();
    if ints.len() as i32 != n {
        Err("incorrect number of digits entered")
    } else {
        Ok(ints)
    }
}

fn calc_checksum(ints: Vec<i32>) -> i32 {
    let mut sum = 0;
    for (ind, val) in ints.iter().enumerate() {
        sum += val * if ind % 2 == 0 { 3 } else { 1 }
    }
    (10 - (sum % 10)) % 10
}

pub fn main() {
    print!("Check (V)alidity or calculate product (C)ode? > ");
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response = response.trim().to_lowercase();
    let choice = response.chars().next();

    if let Some('c') = choice {
        println!("Check digit is {}", calc_checksum(get_n_ints(7).unwrap()))
    } else if let Some('v') = choice {
        println!(
            "This code is {}valid.",
            if calc_checksum(get_n_ints(8).unwrap()) != 0 {
                "not "
            } else {
                ""
            }
        )
    } else {
        panic!("invalid choice")
    }
}
