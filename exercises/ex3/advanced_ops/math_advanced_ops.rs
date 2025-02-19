pub fn square(x: u64) -> u64 {
    x * x
}

pub fn root(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("cannot root negative numbers".to_string())
    } else {
        Ok(x.sqrt())
    }
}

pub fn cube(x: u64) -> u64 {
    x * x * x
}
