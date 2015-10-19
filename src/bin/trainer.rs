extern crate csv;
mod common;

fn train_thetas(learn_rate: f32, theta0: f32, theta1: f32,
                data: Vec<(u32, u32)>) -> (f32, f32) {

    let m = data.len() as f32;

    let mut sum0 = 0.0_f32;
    let mut sum1 = 0.0_f32;
    for (miles, price) in data {

        sum0 += (common::estimate_price(miles, theta0, theta1) as f32 - price as f32) as f32;
        sum1 += ((common::estimate_price(miles, theta0, theta1) as f32 - price as f32) * miles as f32) as f32;
    }
    println!("m: {}, sum0: {}, sum1: {}", m, sum0, sum1);
    (learn_rate * (1_f32 / m) * sum0, learn_rate * (1_f32 / m) * sum1)
}

fn save_thetas(file: &str, theta0: f32, theta1: f32) {

    let mut wtr = csv::Writer::from_file(file).unwrap();

    let result = wtr.encode(("thetas0", "thetas1"));
    assert!(result.is_ok());

    let result = wtr.encode((theta0, theta1));
    assert!(result.is_ok());
}

// https://github.com/SiegeLord/RustGnuplot
fn main() {

    if let Some(filename) = std::env::args().nth(1) {

        // retrieving data
        let mut rdr = csv::Reader::from_file(filename).unwrap();
        let data = rdr.decode().collect::<csv::Result<Vec<(u32, u32)>>>().unwrap();

        let learn_rate = 0.2_f32;
        let (theta0, theta1) = common::retrieve_thetas(common::TMP_FILE_THETAS)
                               .unwrap_or((0_f32, 0_f32));

        let (trained_theta0, trained_theta1) = train_thetas(learn_rate, theta0, theta1, data);
        println!("After computation: ({}, {})", trained_theta0, trained_theta1);

        save_thetas(common::TMP_FILE_THETAS, trained_theta0, trained_theta1);
    }
    else {
        panic!("You need to specify a `csv file`. format: (miles,price)");
    }
}
