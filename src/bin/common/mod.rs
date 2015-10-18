pub fn estimate_price(mileage: u32, theta0: f32, theta1: f32) -> u32 {
    (theta0 + (theta1 * mileage as f32)) as u32
}
