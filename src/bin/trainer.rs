extern crate csv;
extern crate getopts;
extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};
use getopts::Options;
use std::f32;

mod common;

fn train_thetas(learn_rate: f32, theta0: f32, theta1: f32,
                norm_data: &Vec<(f32, f32)>) -> (f32, f32) {

    let m = norm_data.len() as f32;
    let mut sum0 = 0_f32;
    let mut sum1 = 0_f32;

    for &(miles, price) in norm_data {

        let d = (common::estimate_price(miles, theta0, theta1) - price as f32) as f32;
        sum0 += d;
        sum1 += d * miles;
    }
    (learn_rate * (1_f32 / m) * sum0, learn_rate * (1_f32 / m) * sum1)
}

fn loop_train_thetas(learn_rate: f32, theta0: f32, theta1: f32,
                     norm_data: &Vec<(f32, f32)>) -> (f32, f32) {

    let mut theta0 = theta0;
    let mut theta1 = theta1;

    // while error > 0.03_f32 {
    for _ in 0..50_000 {

        let (tmp_theta0, tmp_theta1) = train_thetas(learn_rate, theta0, theta1, &norm_data);
        theta0 -= tmp_theta0;
        theta1 -= tmp_theta1;
    }
    (theta0, theta1)
}

fn save_thetas(file: &str, theta0: f32, theta1: f32) {

    let mut wtr = csv::Writer::from_file(file).unwrap();

    let result = wtr.encode(("theta0", "theta1"));
    assert!(result.is_ok());

    let result = wtr.encode((theta0, theta1));
    assert!(result.is_ok());
}

fn save_min_max(file: &str, min: f32, max: f32) {

    let mut wtr = csv::Writer::from_file(file).unwrap();

    let result = wtr.encode(("min", "max"));
    assert!(result.is_ok());

    let result = wtr.encode((min, max));
    assert!(result.is_ok());
}

fn min_tuple0(data: &Vec<(f32, f32)>) -> f32 {

    let mut ret = f32::MAX;
    for &(val, _) in data {
        if val < ret {
            ret = val;
        }
    }
    ret
}

fn max_tuple0(data: &Vec<(f32, f32)>) -> f32 {

    let mut ret = f32::MIN;
    for &(val, _) in data {
        if val > ret {
            ret = val;
        }
    }
    ret
}

fn normalize_data(data: &Vec<(f32, f32)>, out: &mut Vec<(f32, f32)>, min_max: (f32, f32)) {

    // let min = min_tuple0(data) as f32;
    // let max = max_tuple0(data) as f32;
    let (min, max) = min_max;
    out.clear();
    for &(miles, price) in data {
        out.push( (common::normalize(miles, min, max), price) );
    }
}

fn print_usage(program: &str, opts: Options) {

    let brief = format!("Usage: {} -d DATASET [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("d", "dataset", "the file containing the dataset you want to read", "DATASET");
    opts.optopt("l", "learning-rate", "set the learning rate. default is 0.1", "VALUE");
    opts.optopt("", "tmp-file-thetas", format!("set the file where you want to save thetas values, default {}", common::TMP_FILE_THETAS).as_ref(), "FILE");
    opts.optopt("", "tmp-file-min-max", format!("set the file where you want to save min-max values, default {}", common::TMP_FILE_MIN_MAX).as_ref(), "FILE");
    opts.optflag("g", "graph", "display a graph representing the data and the curve found");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let filename = matches.opt_str("d").unwrap();
    let mut rdr = csv::Reader::from_file(filename).unwrap();
    let data = rdr.decode().collect::<csv::Result<Vec<(f32, f32)>>>().unwrap();

    let min_max = (min_tuple0(&data), max_tuple0(&data));
    let mut norm_data: Vec<(f32, f32)> = Vec::with_capacity(data.len());
    normalize_data(&data, &mut norm_data, min_max);

    let learn_rate: f32 = matches.opt_str("l").unwrap_or("0.1".to_string()).parse().ok().expect("We want a number...");
    let (theta0, theta1) = (0_f32, 0_f32);

    let (trained_theta0, trained_theta1) = loop_train_thetas(learn_rate, theta0, theta1, &norm_data);
    println!("After computation: ({}, {})", trained_theta0, trained_theta1);

    let tmp_file_thetas = matches.opt_str("tmp-file-thetas").unwrap_or(common::TMP_FILE_THETAS.to_string());
    let tmp_file_min_max = matches.opt_str("tmp-file-min-max").unwrap_or(common::TMP_FILE_MIN_MAX.to_string());
    save_thetas(tmp_file_thetas.as_ref(), trained_theta0, trained_theta1);
    save_min_max(tmp_file_min_max.as_ref(), min_max.0, min_max.1);

    if matches.opt_present("g") {

        let x = [0u32, 1, 2];
        let y = [3u32, 4, 5];
        let mut fg = Figure::new();
        fg.axes2d().lines(&x, &y, &[Caption("A line"), Color("black")]);
        fg.show();

        return;
    }
}
