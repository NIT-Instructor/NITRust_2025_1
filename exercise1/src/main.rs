// Task 1: Find Min-Max
fn find_min_max(slice: &[i32]) -> Option<(i32, i32)> {
    if slice.is_empty() {
        return None;
    }
    let min = *slice.iter().min().unwrap();
    let max = *slice.iter().max().unwrap();
    Some((min, max))
}

// Task 2: Vehicle Management System
#[derive(Debug)]
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u16,
    mileage: u32,
    vehicle_type: VehicleType,
}

struct VehicleSystem {
    vehicles: Vec<Vehicle>,
}

impl VehicleSystem {
    fn new() -> Self {
        Self { vehicles: vec![] }
    }

    fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    fn remove_vehicle(&mut self, id: u32) {
        self.vehicles.retain(|v| v.id != id);
    }

    fn list_vehicles(&self) {
        for v in &self.vehicles {
            println!("{:?}", v);
        }
    }

    fn query_by_make(&self, make: &str) {
        for v in &self.vehicles {
            if v.make == make {
                println!("{:?}", v);
            }
        }
    }

    fn query_by_year(&self, year: u16) {
        for v in &self.vehicles {
            if v.year == year {
                println!("{:?}", v);
            }
        }
    }
}

// Task 3: CAN Bus Message Parser
#[derive(Debug)]
enum CanMessage {
    EngineStatus { rpm: u8, coolant_temp: u8, oil_pressure: u8 },
    BrakeSystem { brake_pressure: u8, abs_active: bool },
    Transmission { gear_position: u8, clutch_engaged: bool },
    Unknown,
}

fn parse_can_message(raw: [u8; 8]) -> CanMessage {
    match raw[0] {
        0x01 => CanMessage::EngineStatus { rpm: raw[1], coolant_temp: raw[2], oil_pressure: raw[3] },
        0x02 => CanMessage::BrakeSystem { brake_pressure: raw[1], abs_active: raw[2] != 0 },
        0x03 => CanMessage::Transmission { gear_position: raw[1], clutch_engaged: raw[2] != 0 },
        _ => CanMessage::Unknown,
    }
}

fn process_message(msg: CanMessage) {
    match msg {
        CanMessage::EngineStatus { rpm, coolant_temp, oil_pressure } => {
            println!("EngineStatus - RPM: {}, Coolant Temp: {}, Oil Pressure: {}", rpm, coolant_temp, oil_pressure);
        }
        CanMessage::BrakeSystem { brake_pressure, abs_active } => {
            println!("BrakeSystem - Brake Pressure: {}, ABS Active: {}", brake_pressure, abs_active);
        }
        CanMessage::Transmission { gear_position, clutch_engaged } => {
            println!("Transmission - Gear Position: {}, Clutch Engaged: {}", gear_position, clutch_engaged);
        }
        CanMessage::Unknown => println!("Unknown CAN message received"),
    }
}

fn main() {
    // Testing Task 1
    let numbers = [3, 1, 7, 9, 2];
    if let Some((min, max)) = find_min_max(&numbers) {
        println!("Min: {}, Max: {}", min, max);
    }

    // Testing Task 2
    let mut system = VehicleSystem::new();
    system.add_vehicle(Vehicle { id: 1, make: "Toyota".to_string(), model: "Corolla".to_string(), year: 2020, mileage: 50000, vehicle_type: VehicleType::Car });
    system.add_vehicle(Vehicle { id: 2, make: "Ford".to_string(), model: "F-150".to_string(), year: 2018, mileage: 80000, vehicle_type: VehicleType::Truck });
    system.add_vehicle(Vehicle { id: 3, make: "Toyota".to_string(), model: "Camry".to_string(), year: 2018, mileage: 60000, vehicle_type: VehicleType::Car });
    system.add_vehicle(Vehicle { id: 4, make: "Harley-Davidson".to_string(), model: "Street Glide".to_string(), year: 2019, mileage: 10000, vehicle_type: VehicleType::Motorcycle });
    println!("Number of vehicles: {}", system.vehicles.len());
    system.list_vehicles();
    println!("Query by make: Toyota");
    system.query_by_make("Toyota");
    println!("Query by year: 2018");
    system.query_by_year(2018);
    println!("Removing vehicle with ID 1");
    system.remove_vehicle(1);
    println!("Number of vehicles: {}", system.vehicles.len());
    system.list_vehicles();

    // Testing Task 3
    let messages = [
        [0x01, 0x1F, 0x40, 90, 45, 0, 0, 0], // EngineStatus
        [0x02, 80, 1, 0, 0, 0, 0, 0],       // BrakeSystem
        [0x03, 3, 1, 0, 0, 0, 0, 0],        // Transmission
        [0x04, 1, 1, 1, 1, 1, 1, 1]         // Unknown
    ];
    for msg in messages {
        let parsed_msg = parse_can_message(msg);
        process_message(parsed_msg);
    }
}