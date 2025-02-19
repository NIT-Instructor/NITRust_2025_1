pub trait Vehicle {
    fn start_engine(&self) -> String;
    fn stop_engine(&self) -> String;
}

pub struct Car;
pub struct Truck;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "Car engine started!".to_string()
    }
    fn stop_engine(&self) -> String {
        "Car engine stopped!".to_string()
    }
}

impl Vehicle for Truck {
    fn start_engine(&self) -> String {
        "Truck engine started!".to_string()
    }
    fn stop_engine(&self) -> String {
        "Truck engine stopped!".to_string()
    }
}
