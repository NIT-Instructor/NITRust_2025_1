// Define the CarStatus struct
struct CarStatus {
    speed: f64,      // Speed in KM/H
    fuel_level: f64, // Fuel level as percentage (0 to 100%)
    engine_temp: f64, // Engine temperature in Celsius
}

// Function to display the car's status
fn display_status(car: &CarStatus) {
    println!("Car Status:");
    println!("Speed: {:.2} KM/H", car.speed);
    println!("Fuel Level: {:.2}%", car.fuel_level);
    println!("Engine Temperature: {:.2} C", car.engine_temp);
}

// Function to refill fuel based on the desired percentage
fn refill_fuel(car: &mut CarStatus, fuel_to_add: f64) {
    if fuel_to_add > 100.0 {
        println!("Error: Cannot refill above 100%. Fuel level remains unchanged.");
    } else {
        let available_space = 100.0 - car.fuel_level;
        if fuel_to_add <= available_space {
            car.fuel_level += fuel_to_add;
            println!("Fuel refilled by {:.2}%. New fuel level: {:.2}%", fuel_to_add, car.fuel_level);
        } else {
            println!("Not enough space in the tank. Maximum refill is {:.2}%.", available_space);
        }
    }
}

// Function to set the speed
fn set_speed(car: &mut CarStatus, speed_to_set: f64) {
    car.speed = speed_to_set;
    println!("Speed set to {:.2} KM/H", car.speed);
}

// Function to set the engine temperature
fn set_engine_temperature(car: &mut CarStatus, temperature: f64) {
    car.engine_temp = temperature;
    println!("Engine temperature set to {:.2} C", car.engine_temp);
}

// Main function to test the system
fn main() {
    // Create a new car status instance
    let mut my_car = CarStatus {
        speed: 60.0,        // Initial speed: 60 KM/H
        fuel_level: 50.0,   // Initial fuel level: 50%
        engine_temp: 90.0,  // Initial engine temperature: 90 C
    };

    // Display the initial status
    display_status(&my_car);

    // Refill the fuel
    refill_fuel(&mut my_car, 30.0);  // Try to refill 30% fuel

    // Display the initial status
    display_status(&my_car);

    // Set a new speed
    set_speed(&mut my_car, 120.0); // Set the speed to 120 KM/H

    // Set a new engine temperature
    set_engine_temperature(&mut my_car, 95.0); // Set the engine temperature to 95 C

    // Refill the fuel
    refill_fuel(&mut my_car, 20.0);  // Try to refill 20% fuel

    // Display the updated status
    display_status(&my_car);
}
