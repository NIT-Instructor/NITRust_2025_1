// Task 1: Dashboard Display Manager
#[derive(Debug)]
struct CarStatus {
    speed: u32,
    fuel_level: f32,
    engine_temp: f32,
}

impl CarStatus {
    fn display_status(&self) {
        println!(
            "Speed: {} KM/H | Fuel Level: {:.2}% | Engine Temp: {:.2}C",
            self.speed, self.fuel_level, self.engine_temp
        );
    }

    fn refill_fuel(&mut self, amount: f32) {
        if self.fuel_level + amount > 100.0 {
            println!("Fuel overflow! Tank can only hold up to 100%.");
            self.fuel_level = 100.0;
        } else {
            self.fuel_level += amount;
        }
    }

    fn set_speed(&mut self, new_speed: u32) {
        self.speed = new_speed;
    }

    fn set_engine_temperature(&mut self, new_temp: f32) {
        self.engine_temp = new_temp;
    }
}

// Task 2: Sensor Data Logger
fn log_reading(readings: &mut Vec<f32>, new_value: f32) {
    readings.push(new_value);
}

fn get_average_temperature(readings: &Vec<f32>) -> f32 {
    let sum: f32 = readings.iter().sum();
    sum / readings.len() as f32
}

// Task 3: Implementing a Custom String Struct
struct MyString {
    chars: Vec<char>,
}

impl MyString {
    fn new() -> Self {
        MyString { chars: Vec::new() }
    }

    fn push(&mut self, c: char) {
        self.chars.push(c);
    }

    fn len(&self) -> usize {
        self.chars.len()
    }
}

fn main() {
    // Testing Task 1
    let mut car = CarStatus {
        speed: 0,
        fuel_level: 50.0,
        engine_temp: 90.0,
    };
    car.display_status();
    car.refill_fuel(30.0);
    car.display_status();
    car.set_speed(120);
    car.set_engine_temperature(95.0);
    car.display_status();

    // Testing Task 2
    let mut sensor_readings: Vec<f32> = vec![25.5, 27.3, 26.8];
    println!("Average Temperature: {:.2}", get_average_temperature(&sensor_readings));
    log_reading(&mut sensor_readings, 28.1);
    log_reading(&mut sensor_readings, 29.4);
    println!("New Average Temperature: {:.2}", get_average_temperature(&sensor_readings));

    // Testing Task 3
    let mut my_str = MyString::new();
    my_str.push('H');
    my_str.push('e');
    my_str.push('l');
    my_str.push('l');
    my_str.push('o');
    println!("Custom String Length: {}", my_str.len());
}
