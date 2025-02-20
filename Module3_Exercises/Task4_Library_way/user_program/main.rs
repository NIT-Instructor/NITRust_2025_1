use math_advanced_ops::advanced_ops;
use math_basic_ops::basic_ops;
fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Basic Operations:");
    println!("{} + {} = {}", a, b, basic_ops::add(a, b));
    println!("{} - {} = {}", a, b, basic_ops::subtract(a, b));
    println!("{} * {} = {}", a, b, basic_ops::multiply(a, b));
    println!("{} / {} = {}", a, b, basic_ops::divide(a, b));

    println!("Advanced Operations:");
    println!("Square of {} = {}", a, advanced_ops::square(a));
    println!("Square root of {} = {}", a, advanced_ops::root(a));
    println!("Cube of {} = {}", a, advanced_ops::cube(a));

    //let result1 = basic_ops::divide(12.0,0.0); // Uncomment this to test Panic in basic_ops

    let result2 = advanced_ops::root(-1.4);
}