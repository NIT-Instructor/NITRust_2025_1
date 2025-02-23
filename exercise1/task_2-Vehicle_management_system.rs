#[derive(Debug)]
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[derive(Debug)]
struct Vehicle {
    id: i32,
    vtype: VehicleType,
    make: String,
    model: String,
    year: i32,
    mileage: i64,
}

struct VehicleManager {
    vehicles: Vec<Vehicle>,
}

impl VehicleManager {
    fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }

    fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    fn remove_vehicle(&mut self, id: i32) -> Option<Vehicle> {
        if let Some(pos) = self.vehicles.iter().position(|v| v.id == id) {
            Some(self.vehicles.remove(pos))
        } else {
            None
        }
    }

    fn list_vehicles(&self) {
        for vehicle in &self.vehicles {
            println!("{:?}", vehicle);
        }
    }

    fn query_by_make(&self, make: &str) -> Vec<&Vehicle> {
        let mut result = Vec::new();
        for v in &self.vehicles {
            if v.make == make {
                result.push(v);
            }
        }
        result
    }
}

fn main() {
    println!("task_2-Vehicle_management_system");

    let mut manager = VehicleManager::new();
    manager.add_vehicle(Vehicle {
        vtype: VehicleType::Car,
        id: 1,
        make: String::from("BYD"),
        model: String::from("AATO"),
        year: 2023,
        mileage: 325554,
    });
    manager.add_vehicle(Vehicle {
        id: 2,
        vtype: VehicleType::Car,
        make: String::from("BYD"),
        model: String::from("FOLFIN"),
        year: 2018,
        mileage: 76000,
    });
    manager.list_vehicles();

    let results = manager.query_by_make("BYD");
    println!("Found {} vehicle(s) with make BYD", results.len());

    if let Some(removed) = manager.remove_vehicle(1) {
        println!("Removed: {:?}", removed);
    }

    let results = manager.query_by_make("BYD");
    println!("Found {} vehicle(s) with make BYD", results.len());
}
