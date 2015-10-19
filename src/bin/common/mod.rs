extern crate csv;

pub static TMP_FILE_THETAS: &'static str = "/tmp/thetas.csv";

pub fn estimate_price(mileage: u32, theta0: f32, theta1: f32) -> u32 {
    (theta0 + (theta1 * mileage as f32)) as u32
}

pub fn retrieve_thetas(file: &str) -> Option<(f32, f32)> {

    // retrieving data
    if let Ok(mut rdr) = csv::Reader::from_file(file) {

        if let Ok(ret) = rdr.decode().nth(0).unwrap() {
            return Some(ret);
        }
    }
    None
}
