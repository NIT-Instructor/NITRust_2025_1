struct Car {
    fuel_level: f64,
    is_running: bool,
}

impl Car {
    fn new(fuel_level: f64, is_running: bool) -> Result<Car, String> {
        if fuel_level < 0.0 || fuel_level > 100.0 || is_running {
            Err(String::from("Invalid car initialization parameters."))
        } else {
            Ok(Car {
                fuel_level,
                is_running,
            })
        }
    }

    fn refuel(&mut self, level: f64) {
        if (self.fuel_level + level) <= 100.0 {
            self.fuel_level = self.fuel_level + level; // Cap fuel at 100
        }
        else
        {
            panic!("Cannot refuel anymore tank is full");
        }
    }

    fn start(&mut self) -> Result<(), String> {
        if self.fuel_level > 10.0 {
            if !self.is_running {
                println!("Starting the car now.");
                self.is_running = true;
                Ok(())
            } else {
                Err(String::from("Car is already running."))
            }
        } else {
            Err(String::from("Fuel level is too low to start."))
        }
    }

    fn stop(&mut self) {
        if self.is_running {
            println!("Stopping the car now.");
            self.is_running = false;
        } else {
            println!("Car is already stopped.");
        }
    }
}

fn main() {
    let result = Car::new(3.5, false); // Handle car creation error
    let mut test_car = Car {fuel_level:0.0,is_running:false}; // creating a place holder to be used later
    match result {
        Ok(car_val) => {
            test_car = car_val;
            println!("Car created Successfully");
        },
        Err(e) => println!("Error starting car: {}", e),
    }

    let err = test_car.start();

    match err {
        Ok(_) => println!("No error car started"),
        Err(e) => println!("Error starting car: {}", e),
    }

    test_car.refuel(10.0);

    
    let err1 = test_car.start();
    match err1 {
        Ok(_) => println!("No error car started"),
        Err(e) => println!("Error starting car: {}", e),
    }

    test_car.stop();

    test_car.stop();

    
    let err_val = test_car.start();
    match err_val {
        Ok(_) => println!("No error car started"),
        Err(e) => println!("Error starting car: {}", e),
    }

    test_car.start().expect("Car is already running ");

  // ==== For below code to execute comment out "expect" call above

    // Example of an invalid car creation
    let invalid_car = Car::new(110.0, true);
    match invalid_car {
        Ok(_) => println!("Car created (should not happen)"),
        Err(e) => println!("Error creating car: {}", e),
    }

    test_car.refuel(100.0); // Should give panic
}