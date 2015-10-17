mod common;
use std::io::{self, Read, Write};

fn ask_mileage() -> u32 {

    print!("Enter the mileage you want to estimate: ");
    io::stdout().flush();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("Failed to read stdin");
    buffer = buffer.split_whitespace().nth(0).unwrap_or("Failed to get first word").to_string();
    buffer.parse().ok().expect("Wanted a number")
}

fn retrieve_thetas(file: &str) -> (f32, f32) {

    (0.56f32, 3.038)
}

fn main() {

    let mut thetas_file = "/tmp/thetas.csv".to_string();
    if let Some(arg1) = std::env::args().nth(1) {
        thetas_file = arg1;
    }

    // recovering data
    let (theta0, theta1) = retrieve_thetas(thetas_file.as_ref());
    let mileage = ask_mileage();

    println!("Estimated price is `{}`",
                common::estimate_price(mileage, theta0, theta1));
}
