fn log_reading(readings: &mut Vec<f32>, new_value: f32) {
    readings.push(new_value);
    println!("Logged new reading: {:.2}", new_value);
}

fn get_average_temperature(readings: &Vec<f32>) -> Option<f32> {
    if readings.is_empty() {
        return None;
    }
    
    let sum: f32 = readings.iter().sum();
    Some(sum / readings.len() as f32)
}

fn display_readings(readings: &Vec<f32>) {
    println!("\nCurrent readings:");
    for (index, reading) in readings.iter().enumerate() {
        println!("Reading {}: {:.2}", index + 1, reading);
    }
}

fn main() {
    println!("Sensor Data Logger");
    println!("=================");
    
    // Initialize empty vector for readings
    let mut temperature_readings: Vec<f32> = Vec::new();
    
    // Log first 3 readings
    println!("\nLogging first 3 readings:");
    log_reading(&mut temperature_readings, 23.5);
    log_reading(&mut temperature_readings, 24.0);
    log_reading(&mut temperature_readings, 24.8);
    
    // Display all readings
    display_readings(&temperature_readings);
    
    // Calculate and display average
    match get_average_temperature(&temperature_readings) {
        Some(avg) => println!("\nAverage temperature after 3 readings: {:.2}°C", avg),
        None => println!("No readings available"),
    }
    
    // Log 2 more readings
    println!("\nLogging 2 more readings:");
    log_reading(&mut temperature_readings, 25.2);
    log_reading(&mut temperature_readings, 25.5);
    
    // Display updated readings
    display_readings(&temperature_readings);
    
    // Calculate and display new average
    match get_average_temperature(&temperature_readings) {
        Some(avg) => println!("\nAverage temperature after 5 readings: {:.2}°C", avg),
        None => println!("No readings available"),
    }
}
