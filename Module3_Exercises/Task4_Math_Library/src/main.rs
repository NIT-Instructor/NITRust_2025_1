mod MathLibrary;

fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Basic Operations:");
    println!("{} + {} = {}", a, b, MathLibrary::basic_ops::add(a, b));
    println!("{} - {} = {}", a, b, MathLibrary::basic_ops::subtract(a, b));
    println!("{} * {} = {}", a, b, MathLibrary::basic_ops::multiply(a, b));
    println!("{} / {} = {}", a, b, MathLibrary::basic_ops::divide(a, b));

    println!("Advanced Operations:");
    println!("Square of {} = {}", a, MathLibrary::advanced_ops::square(a));
    println!("Square root of {} = {}", a, MathLibrary::advanced_ops::root(a));
    println!("Cube of {} = {}", a, MathLibrary::advanced_ops::cube(a));

    //let result1 = MathLibrary::basic_ops::divide(12.0,0.0); // Uncomment this to test Panic in basic_ops

    let result2 = MathLibrary::advanced_ops::root(-1.4);
}