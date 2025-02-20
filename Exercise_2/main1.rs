#[derive(Debug)]
#[derive(Clone)]
struct  CarStatus{
    speed: i32,
    fuel_level: f32,
    engine_temp: i32,
}

fn display_status(cs: &CarStatus) {
    println!("printing car status details!!");
    println!("speed of car is: {} KM/H", cs.speed);
    println!("fuel level percent of car is: {:2} %", cs.fuel_level);
    println!("engine temp of car is: {} C", cs.engine_temp);
}

fn refill_fuel(cs: &mut CarStatus, added_fuel: f32) {
    if cs.fuel_level + added_fuel > 100.0 {
        println!("error, could not refill fuel since requested amount is too big");
    }
    else {
        cs.fuel_level+=added_fuel;
    }
}

fn set_speed(cs: &mut CarStatus, speed: i32) {
    cs.speed=speed;
}

fn set_engine_temperature(cs: &mut CarStatus, temperature: i32) {
    cs.engine_temp=temperature;
}


fn main()
{
    
     let mut cs:CarStatus =CarStatus{speed:90,fuel_level:57.9, engine_temp:88};
     display_status(&cs);

     //first failed attempt to set fuel level
     refill_fuel(&mut cs, 60.1);
     //second successfull attempt to set fuel level
     refill_fuel(&mut cs, 10.1);
     //see status after refulling
     display_status(&cs);

     set_speed(&mut cs, 110);
     //see status after setting new speed
     display_status(&cs);

     set_engine_temperature(&mut cs, 49);
     //see status after setting new engine temperature
     display_status(&cs);
}