#[allow(non_snake_case)]
pub trait Convert {
    fn convert(&self) -> f64;
}

pub trait CheckValidRange {
    fn is_valid(value: f64) -> bool {
        if value >= 0.0 && value <= f64::MAX { return true }
        false
    }
}

