use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use std::io::Write;

pub fn main() {
    let contents = read_to_string("details.csv").unwrap();
    let products: HashMap<i32, (&str, f64)> = contents
        .lines()
        .map(|line| {
            let items: Vec<_> = line.splitn(4, ',').collect();
            (
                items[0].parse().unwrap(),
                (items[1], items[2].parse().unwrap()),
            )
        })
        .collect();

    let mut output = String::new();
    let mut cost = 0.0;
    println!(
        "Enter one product per line. Enter the GTIN-8 code, then a
    space, then the quantity. Leave a blank line to stop taking input."
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let vectorised: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        match vectorised.len() {
            0 => break,
            2 => {
                if let Some(v) = products.get(&vectorised[0].parse().unwrap()) {
                    output.push_str(vectorised[0]);
                    output.push_str("\t");
                    output.push_str(&format!("{:20}", v.0));
                    output.push_str("\t");
                    output.push_str(vectorised[1]);
                    output.push_str("\t");
                    output.push_str(&v.1.to_string());
                    output.push_str("\t");
                    let c = v.1 * vectorised[1].parse::<f64>().unwrap();
                    output.push_str(&format!("{:.2}", c));
                    cost += c;
                    output.push_str("\n");
                } else {
                    output.push_str(vectorised[0]);
                    output.push_str("\tproduct not found\n");
                }
            }
            _ => println!("invalid input: please enter 2 values"),
        }
    }
    println!("{}", output);
    println!("Total cost of order\t\t\t\t\t{:.2}", cost)
}
