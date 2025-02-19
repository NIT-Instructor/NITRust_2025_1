struct Car {
    fuel_level: i32,
    is_running: bool,
}

impl Car {
    fn new(fuel_level: i32, is_running: bool) -> Result<Car, String> {
        if fuel_level < 0 || fuel_level > 100 {
            Err("Fuel level must be in valid range (0-100)!".to_string())
        } else if is_running {
            Err("Car cannot be initialized with running state".to_string())
        } else {
            Ok(Car {
                fuel_level,
                is_running,
            })
        }
    }

    fn refuel(&mut self, refuel_amount: i32) {
        self.fuel_level += refuel_amount;
        println!("Car refueled, current level is {}", self.fuel_level)
    }

    fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            Err("Car is already running".to_string())
        } else if self.fuel_level < 10 {
            Err(format!(
                "Car fuel level is to low ({}) - cannot start the car",
                self.fuel_level
            ))
        } else {
            self.is_running = true;
            println!("Car started...");
            Ok(())
        }
    }

    fn stop(&mut self) {
        if !self.is_running {
            println!("Car has already stopped")
        } else {
            self.is_running = false;
            println!("Car stopped!")
        }
    }
}

#[test]
fn car_structure_test() {
    let mut car = Car::new(5, false).unwrap();
    match car.start() {
        Ok(_) => println!("Car started successfully."),
        Err(e) => println!("{}", e),
    }
    car.refuel(50);
    car.stop();
    car.stop();
}
