pub mod advanced_ops {
    pub fn square(a:f64)->f64{
        (a*a)
    }
    
    pub fn root(x: f64) -> Option<f64> {

        //calcuate for postivie number
        if x > 0.0 {
            Some(x.sqrt())
        } else {
            None
        }     
       
    }
    
    fn cube(x:f64)->f64 {
        (x*x*x)
    }
}
