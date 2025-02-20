//TASK5 Error handing of Task2
struct Car{
    fuel_level:i8,//% percentage
    is_running:bool,
}

impl Car {
    
    fn new(fuel_level: i8, is_running: bool) -> Result<Car,String> {
 
            if fuel_level < 0 || fuel_level > 100
            {
                //println!("cannot create car: Invalid Fule Level: {}",fuel_level);
                //Car { fuel_level, is_running }
                Err(String::from("Invalid Fule Level"))
            }
            else if is_running
            {
                // println!("cannot create car with RunningStatus: {}", is_running);
                // Car { fuel_level, is_running }
                Err(String::from("Invalid RunningStatus"))
            }
            else {
                println!("Created car,fuel:{},RunningStatus:{}",fuel_level,is_running);
                Ok(Car { fuel_level, is_running } )
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
    fn start(&mut self) -> Option<String>{
        
        if self.fuel_level < 10 
        {
           // println!("Cannot Start!!! Car is on Low Fule:{}",self.fuel_level);
            return Some(String::from("Low Fuel!!!"));
        }
        else if self.is_running == true
        {
            //println!("Cannot Start!!! Car Already Running!!!")
            return Some(String::from("Alreay Started!!!"));

        }
        else
        {
            println!("Car Started!!!");
            self.is_running = true;
            None
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
    let mut result = Car::new (11,false);
    //let mut result = Car::new (-9,false); //uncomment to check error
    match result {
        Ok(mut car) => 
                    match car.start(){ 
                        Some(e) => {
                                println!("Error Starting car:{}",e);
                        }
                        None => {
                            println!("Car Created & Started success!!!");
                        }
                    },

        Err(e) => println!("Car Create Failed: {}",e),
    }

    result = Car::new (-9,false); ////create with Invalid Fule & check error
    match result {
        Ok(mut car) => 
                    match car.start(){ 
                        Some(e) => {
                                println!("Error Starting car:{}",e);
                        }
                        None => {
                            println!("success");
                        }
                    },

        Err(e) => println!("Car Create Failed: {}",e),
    }

    result = Car::new (9,false); //create with lowfule & check error in start
    match result {
        Ok(mut car) => 
                    match car.start(){ 
                        Some(e) => {
                                println!("Error Starting car:{}",e);
                        }
                        None => {
                            println!("success");
                            //car.refuel
                        }
                    },

        Err(e) => println!("Car Create Failed: {}",e),
    }

          

}