mod common;
use std::io::{self, Read, Write};

fn ask_mileage() -> u32 {

    print!("Enter the mileage you want to estimate: ");
    io::stdout().flush();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("Failed to read stdin");
    buffer = buffer.split_whitespace().nth(0).expect("Failed to get first word").to_string();
    buffer.parse().ok().expect("Wanted a number")
}

fn main() {

    let mut thetas_file = common::TMP_FILE_THETAS.to_string();
    if let Some(arg1) = std::env::args().nth(1) {
        thetas_file = arg1;
    }

    // retrieving data
    let (theta0, theta1) = common::retrieve_thetas(thetas_file.as_ref())
                           .unwrap_or((0_f32, 0_f32));
    let mileage = ask_mileage();

    println!("Estimated price is `{}`",
                common::estimate_price(mileage, theta0, theta1));
}
