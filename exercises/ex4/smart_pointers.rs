use std::{sync::Arc, thread};

fn smart_pointers() {
    let data = Arc::new(vec!['A', 'b', 'C', 'd', 'E', 'f', 'G', 'h', 'I', 'j']);
    let data_clone = Arc::clone(&data);

    let handle1 = thread::spawn(move || {
        for c in data.iter() {
            println!("Thread 1: {c}")
        }
    });

    let handle2 = thread::spawn(move || {
        for c in data_clone.iter() {
            let c_inverted = if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            };
            println!("Thread 2: {}", c_inverted);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn smart_pointers_test() {
    smart_pointers();
}
