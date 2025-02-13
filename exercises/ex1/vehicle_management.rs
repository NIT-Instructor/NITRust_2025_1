#[derive(Debug)]
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[derive(Debug)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u32,
    mileage: f64,
    vehicle_type: VehicleType,
}

struct VehicleManagementSystem {
    vehicles: Vec<Vehicle>,
}

impl VehicleManagementSystem {
    fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    fn remove_vehicle(&mut self, id: u32) {
        if let Some(index) = self.vehicles.iter().position(|v| v.id == id) {
            self.vehicles.remove(index);
        }
    }

    fn list_vehicles(&self) {
        for vehicle in &self.vehicles {
            println!("{:?}", vehicle);
        }
    }

    fn query_vehicles(&self, make: &str, year: u32) {
        let filtered_vehicles: Vec<&Vehicle> = self
            .vehicles
            .iter()
            .filter(|v| v.make == make || v.year == year)
            .collect();

        for vehicle in filtered_vehicles {
            println!("{:?}", vehicle);
        }
    }
}

#[test]
fn vehicle_management_test() {
    let mut vms = VehicleManagementSystem {
        vehicles: Vec::new(),
    };

    vms.list_vehicles();

    vms.add_vehicle(Vehicle {
        id: 1,
        make: String::from("felix"),
        model: String::from("bla"),
        year: 12,
        mileage: 12.0,
        vehicle_type: VehicleType::Car,
    });
    vms.add_vehicle(Vehicle {
        id: 2,
        make: String::from("dor"),
        model: String::from("blabla"),
        year: 13,
        mileage: 13.0,
        vehicle_type: VehicleType::Truck,
    });

    vms.list_vehicles();

    vms.remove_vehicle(1);

    vms.list_vehicles();

    vms.add_vehicle(Vehicle {
        id: 3,
        make: String::from("vectoria"),
        model: String::from("bla"),
        year: 12,
        mileage: 12.0,
        vehicle_type: VehicleType::Car,
    });
    vms.add_vehicle(Vehicle {
        id: 4,
        make: String::from("Joda"),
        model: String::from("blabla"),
        year: 13,
        mileage: 13.0,
        vehicle_type: VehicleType::Truck,
    });

    vms.query_vehicles("vectoria", 13);
}
