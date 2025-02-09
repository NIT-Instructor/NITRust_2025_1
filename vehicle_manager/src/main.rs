
#[derive(Debug, PartialEq)]
pub enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[derive(Debug)]
pub struct Vehicle {
    id: u32,
    vehicle_type: VehicleType,
    producer: String,
    model: String,
    year: u32,
    mileage: f64,
}

pub struct VehicleManagementSystem {
    vehicles: Vec<Vehicle>,
    next_id: u32,
}

impl Vehicle {
    pub fn new(
        vehicle_type: VehicleType,
        producer: String,
        model: String,
        year: u32,
        mileage: f64,
        id: u32,
    ) -> Self {
        Vehicle {
            id,
            vehicle_type,
            producer,
            model,
            year,
            mileage,
        }
    }
}

impl VehicleManagementSystem {
    pub fn new() -> Self {
        VehicleManagementSystem {
            vehicles: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_vehicle(
        &mut self,
        vehicle_type: VehicleType,
        producer: String,
        model: String,
        year: u32,
        mileage: f64,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let vehicle = Vehicle::new(vehicle_type, producer, model, year, mileage, id);
        self.vehicles.push(vehicle);
        id
    }

    pub fn remove_vehicle(&mut self, id: u32) -> bool {
        if let Some(index) = self.vehicles.iter().position(|v| v.id == id) {
            self.vehicles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_all(&self) -> &[Vehicle] {
        &self.vehicles
    }

    pub fn query_by_producer(&self, producer: &str) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|v| v.producer.to_lowercase() == producer.to_lowercase())
            .collect()
    }

    pub fn query_by_year(&self, year: u32) -> Vec<&Vehicle> {
        self.vehicles.iter().filter(|v| v.year == year).collect()
    }

    pub fn get_vehicle_by_id(&self, id: u32) -> Option<&Vehicle> {
        self.vehicles.iter().find(|v| v.id == id)
    }

    pub fn list_all_car_ids(&self) -> Vec<u32> {
        self.vehicles
            .iter()
            .filter(|v| v.vehicle_type == VehicleType::Car)
            .map(|v| v.id)
            .collect()
    }
}

fn main() {
    // Create a new vehicle management system
    let mut vms = VehicleManagementSystem::new();

    // Add some sample vehicles
    let car_id = vms.add_vehicle(
        VehicleType::Car,
        String::from("Toyota"),
        String::from("Corolla"),
        2020,
        15000.0,
    );

    let truck_id = vms.add_vehicle(
        VehicleType::Truck,
        String::from("Ford"),
        String::from("F-150"),
        2019,
        25000.0,
    );

    let _motorcycle_id = vms.add_vehicle(
        VehicleType::Motorcycle,
        String::from("Honda"),
        String::from("CBR"),
        2021,
        5000.0,
    );

    // List all vehicles
    println!("All vehicles:");
    for vehicle in vms.list_all() {
        println!("{:?}", vehicle);
    }

    // Query vehicles by producer
    println!("\nToyota vehicles:");
    for vehicle in vms.query_by_producer("Toyota") {
        println!("{:?}", vehicle);
    }

    // Query vehicles by year
    println!("\nVehicles from 2020:");
    for vehicle in vms.query_by_year(2020) {
        println!("{:?}", vehicle);
    }

    // Get a vehicle by ID
    if let Some(vehicle) = vms.get_vehicle_by_id(truck_id) {
        println!("\nVehicle with ID {}: {:?}", truck_id, vehicle);
    }

    // Remove a vehicle
    if vms.remove_vehicle(car_id) {
        println!("\nSuccessfully removed vehicle with ID: {}", car_id);
    }

    // List remaining vehicles
    println!("\nRemaining vehicles:");
    for vehicle in vms.list_all() {
        println!("{:?}", vehicle);
    }
}