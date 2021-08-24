use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

pub fn main() {
    let contents = read_to_string("stock.csv").unwrap();
    let products: Vec<_> = contents.lines()
    .map(|l| {
        let n = l.splitn(4, ',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        (n[0], n[1], n[2], n[3])
    }).collect();

    let mut file = OpenOptions::new().write(true).create(true).open("orders.csv").unwrap();

    for product in products { // (0=id, 1=stock, 2=threshold, 3=target)
        if product.1 <= product.2 {
            write!(file, "{},{}\n", product.0, product.3-product.1).unwrap(); // id,quantity
        }
    }
}