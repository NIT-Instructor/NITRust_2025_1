use math_basic_ops::basic_ops;
use math_advanced_ops::advanced_ops;

fn main() {

    //using add function from math_basic_ops
    let x = basic_ops::add(5.0,6.0);
    println!("SUM= {}",x);

    //using add function from math_basic_ops
    let sub = basic_ops::subtract(5.0,6.0);
    println!("SUB= {}",sub);

    let z = basic_ops::devide(6.0,2.0);
    println!("Devision= {}",z);

    //using square function from math_advanced_ops
    let y = advanced_ops::square(7.0);
    println!("SQUARE= {}",y);

    // let z = advanced_ops::cube(7.0);  //Uncomment erros as its Private.
    // println!("CUBE= {}",z);

    //Demonstrates Priavate function usage via public function
    let cube = advanced_ops::calc_cube(3.0); 
    println!("CUBE= {}",cube);

}
