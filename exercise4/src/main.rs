use std::sync::{Arc, Mutex};
use std::thread;
use semaphore::Semaphore;

// Task 1: Thread Hello World
fn thread_hello_world() {
    let handle = thread::spawn(|| {
        for _ in 0..5 {
            println!("Hello from the spawned thread");
        }
    });

    for _ in 0..3 {
        println!("Hello from the main thread");
    }

    handle.join().unwrap();
}

// Task 2: Usage of Smart Pointers
fn smart_pointers_example() {
    let data = Arc::new(Mutex::new(vec!['a', 'B', 'c', 'D', 'e']));
    let data_clone = Arc::clone(&data);

    let handle1 = thread::spawn(move || {
        let data = data.lock().unwrap();
        println!("Thread 1: {:?}", *data);
    });

    let handle2 = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data.iter_mut().for_each(|c| {
            *c = if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        });
        println!("Thread 2: {:?}", *data);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// Task 3: Usage of Semaphore
fn semaphore_example() {
    let semaphore = Arc::new(Semaphore::new(1, ()));
    let mut handles = vec![];

    for i in 1..=3 {
        let semaphore_clone = Arc::clone(&semaphore);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _permit = semaphore_clone.try_access();
                println!("T{}", i);
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}


fn main() {
    println!("Task 1: Thread Hello World");
    thread_hello_world();
    
    println!("\nTask 2: Smart Pointers Example");
    smart_pointers_example();

    println!("\nTask 3: Semaphore Example");
    semaphore_example();
}
