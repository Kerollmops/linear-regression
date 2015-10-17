extern crate csv;
mod common;

fn main() {

  if let Some(filename) = std::env::args().nth(1) {

    let mut rdr = csv::Reader::from_file(filename).unwrap().has_headers(true);
    for row in rdr.decode() {

        let (km, price): (u32, u32) = row.unwrap();
        println!("{}, {}", km, price);
    }
  }
  else {
    panic!("You need to specify a `csv file`. format: (km,price)");
  }

  println!("Call of estimate_price: {}",
            common::estimate_price(100, 1.0f32, 1.0f32));
}
