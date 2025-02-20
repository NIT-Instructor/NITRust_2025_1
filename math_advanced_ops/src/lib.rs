pub mod advanced_ops {
    pub fn square(a:f64)->f64 {
        a*a
    }
    
    pub fn root(x: f64) -> f64 {
        x.sqrt()
       
    }
    
    fn cube(x:f64)->f64 {
        x*x*x
    }

    pub fn calc_cube(x:f64)->f64 {
        cube(x)
    }
}
