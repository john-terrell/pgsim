use crate::types::*;

pub trait Atmosphere {
    fn get_temperature(&self, altitude: Altitude<f64>) -> Temperature<f64>;
}
