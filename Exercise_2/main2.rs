fn log_reading(sensor: &mut Vec<f32>, reading:f32) {
    sensor.push(reading);
}

fn get_average_temprature(sensor: &mut Vec<f32>) -> f32 {
    let mut sum:f32=0.0;
    let mut size:f32=0.0;
    for i in 0..sensor.len() {
        sum+=sensor[i];
        size+=1.0;
    }
    println!("sum is: {}", sum);
    let avarage:f32 = sum/size;
    avarage
}

fn main()
{
     
     let mut logger:Vec<f32> = Vec::new();
     log_reading(&mut logger, 80.0);
     log_reading(&mut logger, 75.0);
     log_reading(&mut logger, 90.0);
     let mut av:f32=get_average_temprature(&mut logger);
     println!("avarage log is on temprature: {:2}", av);
     
     //now add 2 more readings and print avarage again
     log_reading(&mut logger, 51.0);
     log_reading(&mut logger, 120.0);
     av=get_average_temprature(&mut logger);
     println!("avarage log is on temprature: {:2}", av);
}