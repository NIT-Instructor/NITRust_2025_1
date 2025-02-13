fn log_reading(prev_reads: &mut Vec<f32>, read_to_add: f32) {
    prev_reads.push(read_to_add);
}

fn get_average_temperature(prev_reads: &Vec<f32>) -> f32 {
    let read_count = prev_reads.len();
    if read_count == 0 {
        0.0
    } else {
        prev_reads.iter().sum::<f32>() / read_count as f32
    }
}

#[test]
fn sensor_data_logger_test() {
    let mut v = vec![];

    log_reading(&mut v, 1.0);
    log_reading(&mut v, 2.0);
    log_reading(&mut v, 3.0);
    let v_avg = get_average_temperature(&v);
    println!("Avg of {:?} is {v_avg}", v);

    log_reading(&mut v, 4.0);
    log_reading(&mut v, 5.0);
    let v_avg = get_average_temperature(&v);
    println!("Avg of {:?} is {v_avg}", v);
}
