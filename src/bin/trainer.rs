extern crate csv;
mod common;

fn train_thetas(learn_rate: f32, thetas: (f32, f32),
                data: Vec<(u32, u32)>) -> (f32, f32) {

    let (mut theta0, mut theta1) = thetas;
    let m = data.len();
    for (miles, price) in data {

        // let tmp0 = learn_rate * (1 / m) * (sum (estimatePrice(mileage[i]) − price[i]), i=0, m-1)
        // let tmp1 = learn_rate * (1 / m) * (sum (estimatePrice(mileage[i]) − price[i]) * mileage[i], i=0, m-1)

        // theta0 = tmp0;
        // theta1 = tmp1;
        // println!("{}, {}", miles, price);
    }

// common::estimate_price(100, 1.0f32, 1.0f32);
/*
tmp_theta_0 = learningRate * (1/m) * (sum (estimatePrice(mileage[i]) − price[i]), i=0, m-1)
tmp_theta_1 = learningRate * (1/m) * (sum (estimatePrice(mileage[i]) − price[i]) * mileage[i], i=0, m-1)
*/

    //
    (0.0_f32, 0.0_f32)
}

// https://github.com/SiegeLord/RustGnuplot
fn main() {

  if let Some(filename) = std::env::args().nth(1) {

    // retrieving data
    let mut rdr = csv::Reader::from_file(filename).unwrap().has_headers(true);
    let data = rdr.decode().collect::<csv::Result<Vec<(u32, u32)>>>().unwrap();

    // compute thetas
    let learn_rate = 0.2_f32;

    // if theta not created then write zero in theta file...
    let start_thetas = (0.0_f32, 0.0_f32);


    let (theta0, theta1) = train_thetas(learn_rate, start_thetas, data);
    println!("After computation: ({}, {})", theta0, theta1);

    // save computed thetas
    // ...
  }
  else {
    panic!("You need to specify a `csv file`. format: (miles,price)");
  }
}
