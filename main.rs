//-------------E2-Task1---------------------------------------------//

struct CarStatus {
    speed:u8,
    fuel_level:u8,
    engine_temp:f32
}

fn display_status(carstat:&CarStatus) {

    println!("display_status!");

    println!("speed: {},fuel_level: {}%,engine_temp: {} degree Celcius",
        carstat.speed,carstat.fuel_level,carstat.engine_temp);

}


fn refill_fuel(carstat:&mut CarStatus,desiredRefillPercent:u8 ) {
    
    println!("refill_fuel!");

    if carstat.fuel_level + desiredRefillPercent <= 100 {
        let filledPercent = carstat.fuel_level + desiredRefillPercent;
        carstat.fuel_level = filledPercent;
    }
    else {
        println!("NO TANK CAPACITY!!!");
    }
    
}

fn set_speed(carstat:&mut CarStatus,desiredSpeed:u8) {

    println!("set_speed!");

    carstat.speed = desiredSpeed;
}

fn set_engine_temperature(carstat:&mut CarStatus,desiredEngTemp:f32) {

    println!("set_engine_temperature!");

    carstat.engine_temp = desiredEngTemp;
}
//-------------E2-Task1 END---------------------------------------------//

//-----E2-Task2-Sensor data logger--------------------------//

fn log_reading(logr :&mut Vec<f32>,val:f32)
{
    logr.push(val);
}

fn get_average_temperature(logr : &mut Vec<f32>) -> (f32)
{
    let size: f32 = logr.len() as f32;
    let mut total = 0.0;
    for val in logr {
            total = total + *val;
        }
   return total/size ;
}
//-----E2-Task2--------------------------//

//---------------------EXCERISE-2----------------------------------//
fn main()
{
//-------------E2-Task1-The Dashboard Display Manager---------------//
    let mut stat1 = CarStatus{
        speed:20,
        fuel_level:25,
        engine_temp:30.2
    };

    display_status(&stat1);
    refill_fuel(&mut stat1,25);
    display_status(&stat1);

    set_speed(&mut stat1,50);
    display_status(&stat1);

    set_engine_temperature(&mut stat1,60.8);
    display_status(&stat1);

//-------------E2-Task1---------------------------------------------//

//-----E2-Task2-Sensor data logger--------------------------//

let mut logger: Vec<f32> = vec![];

//-----E2-Task2-------------------------//
log_reading(&mut logger,2.0);
log_reading(&mut logger,3.0);
log_reading(&mut logger,5.0);

let avg = get_average_temperature(&mut logger);
println!("average-1= {}", avg);

log_reading(&mut logger,7.0);
log_reading(&mut logger,9.0);

let avg2 = get_average_temperature(&mut logger);
println!("average-2= {}", avg2);

}