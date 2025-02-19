pub fn square(a: f64) -> f64 {
    a * a
}

pub fn root(a: f64) -> Result<f64, &'static str> {
    if a < 0.0 {
        Err("Cannot compute the root of a negative number")
    } else {
        Ok(a.sqrt())
    }
}

pub fn cube(a: f64) -> f64 {
    a * a * a
}
