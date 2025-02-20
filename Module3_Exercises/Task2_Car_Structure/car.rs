struct Car{
    fuel_level : f64,
    is_running : bool,
}

impl Car{

    fn new(fuellevel: f64 , isrunning : bool )-> Car {
        Car{fuel_level:fuellevel, is_running:isrunning}
    }

    fn refuel(&mut self,level:f64){
        self.fuel_level = self.fuel_level+level;
    }

    fn start(&mut self){
        if self.fuel_level > 10.0
         {
            if self.is_running == false{
                println!("Starting the car now ");
                self.is_running = true;
            }
            else
            {
                println!("Car is already running ");
            }
        }
        else
        {
            println!("Fule level is low");
        }
    }

    fn stop(&mut self)
    {
        if self.is_running == true{
            println!("Stopping the car now ");
            self.is_running = false;
        }
        else
        {
            println!("Car is already stopped ");
        }

    }
}

fn main()
{
    let mut test_car = Car::new(3.5,false);

    test_car.start();

    test_car.refuel(10.0);

    test_car.start();

    test_car.stop();

    test_car.stop();

    test_car.start();

    test_car.start();
}