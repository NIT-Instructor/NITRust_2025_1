use semaphore::Semaphore;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn semaphore() {
    let sem = Arc::new(Semaphore::new(1, ()));
    let current_id = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    let thread_count = 3;
    for i in 1..=thread_count {
        let sem_clone = Arc::clone(&sem);
        let current_id_clone = Arc::clone(&current_id);

        let handle = thread::spawn(move || {
            for count in 1..=100 {
                loop {
                    match sem_clone.try_access() {
                        Err(_e) => thread::yield_now(),
                        Ok(_) => {
                            let mut id = current_id_clone.lock().unwrap();
                            if *id == i {
                                println!("{count} - T{i}: {i} ");
                                *id = if i == thread_count { 1 } else { i + 1 };
                                break;
                            }
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn semaphore_test() {
    semaphore();
}
