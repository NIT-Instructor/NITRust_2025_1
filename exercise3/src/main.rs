mod vehicle;
mod car;
mod battery;

use vehicle::{Car, Truck, Vehicle};
use car::CarStruct;
use battery::Battery;
use math_ops::basic_ops;
use math_ops::advanced_ops;

fn main() {
    // Task 1: Vehicle Trait Test
    let car = Car;
    let truck = Truck;
    println!("{}", car.start_engine());
    println!("{}", car.stop_engine());
    println!("{}", truck.start_engine());
    println!("{}", truck.stop_engine());

    // Task 2 & 5: CarStruct Test
    match CarStruct::new(50.0, false) {
        Ok(mut my_car) => {
            my_car.refuel(20.0);
            match my_car.start() {
                Ok(()) => println!("Car started successfully."),
                Err(e) => println!("Error: {}", e),
            }
            my_car.stop();

            // Testing invalid refueling
            my_car.refuel(-10.0); // Should not reduce fuel level
            match my_car.start() {
                Ok(()) => println!("Car started successfully."),
                Err(e) => println!("Error (expected): {}", e),
            }
            
            my_car.refuel(100.0); // Exceed max fuel level
            match my_car.start() {
                Ok(()) => println!("Car started successfully."),
                Err(e) => println!("Error (unexpected): {}", e),
            }
        }
        Err(e) => println!("Car initialization failed: {}", e),
    }

    // Task 3: Battery Test
    let battery = Battery {
        charge_level: 100.0,
        is_disposed: false,
    };
    let battery = battery.use_up();
    battery.dispose();

    // Task 4 & 6: Math Operations Test
    println!("Addition: {}", basic_ops::add(10.0, 5.0));
    println!("Subtraction: {}", basic_ops::subtract(10.0, 5.0));
    println!("Multiplication: {}", basic_ops::multiply(10.0, 5.0));
    println!("Multiplication: {}", basic_ops::multiply(10.0, 5.0));
    println!("Square: {}", advanced_ops::square(2.0));
    println!("Cube: {}", advanced_ops::cube(2.0));
    match basic_ops::divide(10.0, 0.0) {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match basic_ops::divide(10.0, 5.0) {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match advanced_ops::root(4.0) {
        Ok(result) => println!("Square Root: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match advanced_ops::root(-4.0) {
        Ok(result) => println!("Square Root: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
