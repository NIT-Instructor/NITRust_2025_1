use std::thread;

fn thread_hello_world() {
    let handler = thread::spawn(|| {
        for _ in 0..5 {
            println!("Hello from the spawned thread")
        }
    });
    handler.join().unwrap();
    for _ in 0..3 {
        println!("Hello from the main thread")
    }
}

#[test]
fn thread_hello_world_test() {
    thread_hello_world();
}
