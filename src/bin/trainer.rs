extern crate csv;

fn main() {

  if let Some(file) = std::env::args().nth(1) {

    let mut rdr = csv::Reader::from_file(file).unwrap().has_headers(true);
    for row in rdr.decode() {

        let (km, price): (u32, u32) = row.unwrap();
        println!("{}, {}", km, price);
    }
  }
  else {
    panic!("You need to specify a `csv file`. format: (km,price)");
  }
}
