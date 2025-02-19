// Create a simulated car struct with refuel, start and stop methods. 
struct Car{
    fuel_level:u8,//% percentage
    is_running:bool,
}

impl Car {
    
    fn new(fuel_level: u8, is_running: bool) -> Car {
            println!("Creating car,fuel:{},RunningStatus:{}",fuel_level,is_running);
            Car { fuel_level, is_running }
        }

    fn refuel(&mut self, desiredRefill:u8) {

        if self.fuel_level + desiredRefill <= 100 {
            let filledPercent = self.fuel_level + desiredRefill;
            self.fuel_level = filledPercent;
            println!("Car Refueld to: {}", self.fuel_level);

        }
        else {
            println!("Curent Fuel Leve {}%",self.fuel_level);
            println!("NO TANK CAPACITY to Refuel!!! desiredAmount: {}%",desiredRefill);
        }
    }
    fn start(&mut self){
        if self.fuel_level > 10 && !self.is_running
        {
            println!("Car Started!!!");
            self.is_running = true;
        }
        else if self.fuel_level <= 10 
        {
            println!("Cannot Start!!! Car is on Low Fule:{}",self.fuel_level);
        }
        else if self.is_running
        {
            println!("Cannot Start!!! Car Already Running!!!")
        }

    }
    fn stop(&mut self){
        if self.is_running 
        {
            println!("Car Stopped!!!");
            self.is_running = false;

        }
        else if self.is_running == false 
        {
            println!("Cannot Stop Car!!!  Already Stopped!!!");
        }

    }
}
//impl contractor
fn main()
{
    let mut car = Car::new (30,false);
    car.start();
    car.stop();
    car.refuel(20);
    //car = Car::new (30,true);
    car.start();
    car.stop();
    car.refuel(60);

}