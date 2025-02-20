pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

pub fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

pub fn divide(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        panic!("Error Trying to divide by zero");
    }
    x / y
}
