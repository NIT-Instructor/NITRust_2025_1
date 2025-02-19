pub trait VehicleTrait {
    fn start_engine(&self) -> &str;
    fn stop_engine(&self) -> &str;
}

struct Car {}
struct Truck {}

impl VehicleTrait for Car {
    fn start_engine(&self) -> &str {
        "Starting the car..."
    }

    fn stop_engine(&self) -> &str {
        "Car Stopped!"
    }
}

impl VehicleTrait for Truck {
    fn start_engine(&self) -> &str {
        "Starting the truck..."
    }

    fn stop_engine(&self) -> &str {
        "Truck Stopped!"
    }
}

#[test]
fn vehicle_trait_test() {
    let car = Car {};
    println!("{}", car.start_engine());

    let truck = Truck {};
    println!("{}", truck.start_engine());

    println!("{}", car.stop_engine());
    println!("{}", truck.stop_engine());
}
