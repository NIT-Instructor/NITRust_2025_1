// Function to log a new reading to the vector
fn log_reading(readings: &mut Vec<f32>, new_value: f32) {
    readings.push(new_value);
    println!("New reading logged: {:.2}", new_value);
}

// Function to calculate the average temperature from the logged readings
fn get_average_temperature(readings: &[f32]) -> f32 {
    let sum: f32 = readings.iter().sum();
    let count = readings.len();
    if count > 0 {
        sum / count as f32
    } else {
        0.0 // Return 0.0 if no readings are logged
    }
}

// Main function to test the logging and average calculation
fn main() {
    // Create a vector to store the sensor readings (initially empty)
    let mut sensor_readings: Vec<f32> = Vec::new();

    // Log the first 3 readings
    log_reading(&mut sensor_readings, 22.5);
    log_reading(&mut sensor_readings, 23.0);
    log_reading(&mut sensor_readings, 21.8);

    // Calculate and print the average of the first 3 readings
    let average = get_average_temperature(&sensor_readings);
    println!("Average temperature of first 3 readings: {:.2} C", average);

    // Log 2 more readings
    log_reading(&mut sensor_readings, 24.1);
    log_reading(&mut sensor_readings, 22.9);

    // Calculate and print the updated average
    let new_average = get_average_temperature(&sensor_readings);
    println!("New average temperature after 2 more readings: {:.2} C", new_average);
}
