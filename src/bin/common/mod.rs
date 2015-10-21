pub static TMP_FILE_THETAS: &'static str = ".thetas_values.csv";
pub static TMP_FILE_MIN_MAX: &'static str = ".min_max_values.csv";

pub fn normalize(data: f32, min: f32, max: f32) -> f32 {
    (data - min) / (max - min)
}

pub fn estimate_price(mileage: f32, theta0: f32, theta1: f32) -> f32 {
    (theta0 + (theta1 * mileage))
}
