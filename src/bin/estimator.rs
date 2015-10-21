extern crate csv;
use std::io::{self, Write};
use std::f32;
mod common;

fn ask_mileage() -> u32 {

    print!("Enter the mileage you want to estimate: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("Failed to read stdin");
    buffer = buffer.split_whitespace().nth(0).expect("Failed to get first word").to_string();
    buffer.parse().ok().expect("Wanted a positive number")
}

pub fn retrieve_thetas(file: &str) -> Option<(f32, f32)> {

    if let Ok(mut rdr) = csv::Reader::from_file(file) {

        if let Ok(ret) = rdr.decode().nth(0).unwrap() {
            return Some(ret);
        }
    }
    None
}

pub fn retrieve_min_max(file: &str) -> Option<(f32, f32)> {

    if let Ok(mut rdr) = csv::Reader::from_file(file) {

        if let Ok(ret) = rdr.decode().nth(0).unwrap() {
            return Some(ret);
        }
    }
    None
}

fn main() {

    let mut thetas_file = common::TMP_FILE_THETAS.to_string();
    if let Some(arg1) = std::env::args().nth(1) {
        thetas_file = arg1;
    }

    // retrieving data
    let (theta0, theta1) = retrieve_thetas(thetas_file.as_ref()).unwrap_or((0_f32, 0_f32));
    let (min, max) = retrieve_min_max(common::TMP_FILE_MIN_MAX).unwrap_or((0_f32, f32::INFINITY));
    let normed_mileage = common::normalize(ask_mileage() as f32, min, max);

    let estimated = common::estimate_price(normed_mileage, theta0, theta1);
    println!("Estimated price is `{}`", estimated);
}
