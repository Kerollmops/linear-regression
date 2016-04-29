extern crate csv;
extern crate getopts;

use getopts::Options;
use std::io::{self, Write};
use std::f32;

mod common;

fn ask_mileage() -> f32 {

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

fn print_usage(program: &str, opts: Options) {

    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("m", "mileage", "set directly the mileage you want to estimate, default `stdin`", "VALUE");
    opts.optopt("", "tmp-file-thetas", format!("set the file where you want to read thetas values, default {}", common::TMP_FILE_THETAS).as_ref(), "FILE");
    opts.optopt("", "tmp-file-min-max", format!("set the file where you want to read min-max values, default {}", common::TMP_FILE_MIN_MAX).as_ref(), "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let thetas_file = matches.opt_str("tmp-file-thetas").unwrap_or(common::TMP_FILE_THETAS.to_string());
    let min_max_file = matches.opt_str("tmp-file-min-max").unwrap_or(common::TMP_FILE_MIN_MAX.to_string());

    // retrieving data
    let (theta0, theta1) = retrieve_thetas(thetas_file.as_ref()).unwrap_or((0_f32, 0_f32));
    let (min, max) = retrieve_min_max(min_max_file.as_ref()).unwrap_or((0_f32, f32::INFINITY));

    let mileage: f32 = match matches.opt_str("m") {
        Some(value) => value.parse().ok().expect("Option `-m` need a number..."),
        None => ask_mileage()
    };

    let normed_mileage = common::normalize(mileage, min, max);

    let estimated = common::estimate_price(normed_mileage, theta0, theta1);
    println!("Estimated price is `{}`", estimated);
}
