fn main() {
    println!("Hello, world!");
}
#[derive(Debug)]
struct CarStatus {
    speed: f64,
    fuel_level: f64,
    engine_temp: f64,
}

impl CarStatus {
    fn new() -> CarStatus {
        CarStatus {
            speed: 0.0,
            fuel_level: 0.0,
            engine_temp: 0.0,
        }
    }
}

fn display_status(status: &CarStatus) {
    println!("Current Car Status:");
    println!("Speed: {:.2} KM/H", status.speed);
    println!("Fuel Level: {:.2}%", status.fuel_level);
    println!("Engine Temperature: {:.2}°C", status.engine_temp);
    println!("------------------------");
}

fn refill_fuel(status: &mut CarStatus, amount: f64) -> Result<(), String> {
    if amount < 0.0 {
        return Err("Fuel amount cannot be negative".to_string());
    }
    
    let new_level = status.fuel_level + amount;
    if new_level > 100.0 {
        return Err("Cannot overfill the tank".to_string());
    }
    
    status.fuel_level = new_level;
    Ok(())
}

fn set_speed(status: &mut CarStatus, speed: f64) -> Result<(), String> {
    if speed < 0.0 {
        return Err("Speed cannot be negative".to_string());
    }
    
    status.speed = speed;
    Ok(())
}

fn set_engine_temperature(status: &mut CarStatus, temp: f64) -> Result<(), String> {
    if temp < -50.0 || temp > 200.0 {
        return Err("Temperature out of reasonable range (-50°C to 200°C)".to_string());
    }
    
    status.engine_temp = temp;
    Ok(())
}

fn main() {
    println!("Car Dashboard Display System Test");
    println!("================================");
    
    // Create a new car status
    let mut car = CarStatus::new();
    
    // Initial status
    println!("\nInitial status:");
    display_status(&car);
    
    // Test refilling fuel
    println!("\nTesting fuel refill:");
    match refill_fuel(&mut car, 75.5) {
        Ok(_) => println!("Successfully added fuel"),
        Err(e) => println!("Error: {}", e),
    }
    display_status(&car);
    
    // Try to overfill - should get error
    println!("\nTesting fuel overfill:");
    match refill_fuel(&mut car, 30.0) {
        Ok(_) => println!("Successfully added fuel"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test setting speed
    println!("\nTesting speed setting:");
    match set_speed(&mut car, 60.5) {
        Ok(_) => println!("Successfully set speed"),
        Err(e) => println!("Error: {}", e),
    }
    display_status(&car);
    
    // Test setting engine temperature
    println!("\nTesting engine temperature setting:");
    match set_engine_temperature(&mut car, 90.5) {
        Ok(_) => println!("Successfully set engine temperature"),
        Err(e) => println!("Error: {}", e),
    }
    display_status(&car);
    
    // Test invalid temperature
    println!("\nTesting invalid temperature:");
    match set_engine_temperature(&mut car, 250.0) {
        Ok(_) => println!("Successfully set engine temperature"),
        Err(e) => println!("Error: {}", e),
    }
}