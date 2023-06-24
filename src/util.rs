use std::f64::consts::PI;

use rand::Rng;

// Utility
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_bounded(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
