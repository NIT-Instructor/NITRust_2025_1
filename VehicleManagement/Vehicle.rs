#[derive(Debug, Clone)]
struct Vehicle {
    id: u64,
    make: String,
    model: String,
    year: u32,
    milage: f64,
}

#[allow(dead_code)]
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

#[allow(dead_code)]
fn add_new_vehicle(vehicle_repo: &mut Vec<Vehicle>, vehicle_details: Vehicle) {
    vehicle_repo.push(vehicle_details);
}

#[allow(dead_code)]
fn remove_vehicle_with_id(vehicle_repo: &mut Vec<Vehicle>, id: u64) {
    let mut index = 0;
    while index < vehicle_repo.len() {
        if vehicle_repo[index].id == id {
            vehicle_repo.remove(index);
        } else {
            index += 1;
        }
    }
}

#[allow(dead_code)]
fn query_by_make(vehicle_repo: &Vec<Vehicle>, make: &str) -> Vehicle {
    for vehicle in vehicle_repo {
        if vehicle.make == make {
            return vehicle.clone();
        }
    }

    panic!("Vehicle with make '{}' not found", make); // Panic if no match is found
}

fn main() {
    let mut vehiclerepo: Vec<Vehicle> = Vec::new();
    let new_veh1 = Vehicle {
        id: 12,
        make: "ford".to_string(),
        model: "t".to_string(),
        year: 2021,
        milage: 3.0,
    };

    add_new_vehicle(&mut vehiclerepo, new_veh1);

    let new_veh2 = Vehicle {
        id: 14,
        make: "PAL".to_string(),
        model: "t".to_string(),
        year: 2021,
        milage: 3.0,
    };

    add_new_vehicle(&mut vehiclerepo, new_veh2);

    // Print all vehicles
    for veh in vehiclerepo.iter() {
        println!("Vehicle id : {} , Make : {} , Model: {}, year {}, milage {}",veh.id, veh.make, veh.model, veh.year, veh.milage);
    }

    println!("Length {}", vehiclerepo.len());

    // Query a vehicle by make
    let make_veh = query_by_make(&vehiclerepo, "ford");

    println!("Details of Ford vehicle is {} {} {} {} {}",make_veh.id, make_veh.make, make_veh.model, make_veh.year, make_veh.milage);

    println!("Removing the vehicle with id 14");
    // Remove vehicle by ID
    remove_vehicle_with_id(&mut vehiclerepo, 14);

    println!("List after removal");
    // Print remaining vehicles
    for veh in vehiclerepo.iter() {
        println!("Vehicle id : {} , Make : {} , Model: {}, year {}, milage {}",veh.id, veh.make, veh.model, veh.year, veh.milage);
    }

    println!("Length {}", vehiclerepo.len());
}
