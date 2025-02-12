struct CarStatus {
    speed: f32,
    fuel: f32,
    engine_temp: f32,
}

fn display_status(car_status: &CarStatus) {
    println!(
        "Car Status: speed = {} KM/H | fuel = {:.2}% | engine temp = {}°C",
        car_status.speed, car_status.fuel, car_status.engine_temp
    );
}

fn refill_tank(car_status: &mut CarStatus, refill_amount: f32) {
    car_status.fuel += refill_amount;
    if car_status.fuel > 100.0 {
        car_status.fuel = 100.0;
    }
}

fn set_speed(car_status: &mut CarStatus, wanted_speed: f32) {
    car_status.speed = wanted_speed;
}

fn set_engine_temperature(car_status: &mut CarStatus, wanted_tmp: f32) {
    car_status.engine_temp = wanted_tmp;
}

#[test]
fn dashboard_display_manager_test() {
    let mut car_status = CarStatus {
        speed: 87.0,
        fuel: 25.0,
        engine_temp: 60.0,
    };

    display_status(&car_status);

    refill_tank(&mut car_status, 20.0);
    set_speed(&mut car_status, 20.0);
    set_engine_temperature(&mut car_status, 20.0);

    display_status(&car_status);

    assert!(car_status.fuel == 20.0 + 25.0);
    assert!(car_status.engine_temp == 20.0);
    assert!(car_status.speed == 20.0);
}
