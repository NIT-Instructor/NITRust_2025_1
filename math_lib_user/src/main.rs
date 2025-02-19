use math_basic_ops::basic_ops;
use math_advanced_ops::advanced_ops;

fn main() {

    //using add function from math_basic_ops
    let x = basic_ops::add(5.0,6.0);
    println!("SUM= {}",x);

    //using square function from math_advanced_ops

    let y = advanced_ops::square(7.0);
    println!("SQUARE= {}",y);
}
