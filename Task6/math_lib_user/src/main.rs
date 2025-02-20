use math_basic_ops::basic_ops;
use math_advanced_ops::advanced_ops;

fn main() {

 //Demonstrating Error handling

    let z = basic_ops::devide(6.0,2.0);
   // println!("Devicd= {}",z);
    match z {
        Ok(z) => println!("Result: {}",z),
        Err(e) => println!("Error: {}",e),

    }
    //Devide by zero error
    let result = basic_ops::devide(6.0,0.0);
    // println!("Devicd= {}",z);
     match result {
         Ok(result) => println!("Result of devision: {}",result),
         Err(e) => println!("Error: {}",e), 
     }


     let mut y = advanced_ops::root(7.0);

     match y {
        Some(y)=> println!("Root of number is: {}",y),
        None => println!("Error:Cannot calculate root for negative number"),
     }

     y = advanced_ops::root(-7.0);
     match y {
        Some(y)=> println!("Root of number is: {}",y),
        None => println!("Error:Cannot calculate root for negative number"),
     }
}
