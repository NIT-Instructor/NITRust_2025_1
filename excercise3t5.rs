//TASK5 Error handing of Task2
struct Car{
    fuel_level:i8,//% percentage
    is_running:bool,
}

impl Car {
    
    fn new(fuel_level: i8, is_running: bool) -> Car {
            // if fuel_level > 0 && !is_running
            // {
            //     println!("Created car,fuel:{},RunningStatus:{}",fuel_level,is_running);
            //     Car { fuel_level, is_running }
            // }
            if fuel_level < 0 || fuel_level > 100
            {
                println!("cannot create car: Invalid Fule Level: {}",fuel_level);
                Car { fuel_level, is_running }
            }
            else if is_running
            {
                println!("cannot create car with RunningStatus: {}", is_running);
                Car { fuel_level, is_running }
            }
            else {
                println!("Created car,fuel:{},RunningStatus:{}",fuel_level,is_running);
                Car { fuel_level, is_running } 
            }
        }

    fn refuel(&mut self, desiredRefill:i8) {

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
    let mut car = Car::new (0,false);
    car.start();

    car.stop();

    let mut car2 = Car::new (-2,false); //indicates invalid fuel level

    let mut car3 = Car::new (5,true);
    car3.start();

}