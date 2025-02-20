pub mod advanced_ops {
    pub fn square(val: f64) -> f64 {
        val * val
    }
    
    pub fn root(val: f64) -> f64 {
        if val < 0.0 {
            panic!("Cannot take the square root of a negative number!");
        }
        val.sqrt()
    }
    
    pub fn cube(val: f64) -> f64 {
        val * val * val
    }
}