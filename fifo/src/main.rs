#[derive(Debug)]
struct FifoBuffer {
    data: Vec<u8>,
}

impl FifoBuffer {
    // Create a new empty buffer 
    fn new() -> FifoBuffer {
        FifoBuffer {
            data: Vec::new(),
        }
    }

    // Add an element to the back of the buffer
    fn enqueue(&mut self, value: u8) {
        self.data.push(value);
    }

    // Remove and return the first element from the buffer
    fn dequeue(&mut self) -> Option<u8> {
        if !self.is_empty() {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    // Look at the first element without removing it
    fn peek(&self) -> Option<&u8> {
        self.data.first()
    }

    // Check if the buffer is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the current size of the buffer
    fn size(&self) -> usize {
        self.data.len()
    }

    // Clear all elements from the buffer
    fn clear(&mut self) {
        self.data.clear();
    }

    // Display the current contents of the buffer
    fn display(&self) {
        println!("Buffer contents: {:?}", self.data);
        println!("Buffer size: {}", self.size());
    }
}

fn main() {
    println!("FIFO Buffer Test");
    println!("================\n");

    // Create a new buffer
    let mut buffer = FifoBuffer::new();
    println!("Created new empty buffer:");
    buffer.display();

    // Test enqueue operations
    println!("\nTesting enqueue operations:");
    buffer.enqueue(1);
    buffer.enqueue(2);
    buffer.enqueue(3);
    buffer.display();

    // Test peek operation
    println!("\nTesting peek operation:");
    match buffer.peek() {
        Some(value) => println!("Front element: {}", value),
        None => println!("Buffer is empty"),
    }
    buffer.display();  // Should show same contents as before

    // Test dequeue operations
    println!("\nTesting dequeue operations:");
    match buffer.dequeue() {
        Some(value) => println!("Dequeued: {}", value),
        None => println!("Buffer is empty"),
    }
    buffer.display();

    // Dequeue another element
    match buffer.dequeue() {
        Some(value) => println!("Dequeued: {}", value),
        None => println!("Buffer is empty"),
    }
    buffer.display();

    // Test empty buffer behavior
    println!("\nTesting until empty:");
    match buffer.dequeue() {
        Some(value) => println!("Dequeued last element: {}", value),
        None => println!("Buffer is empty"),
    }
    buffer.display();

    // Test dequeue on empty buffer
    println!("\nTesting dequeue on empty buffer:");
    match buffer.dequeue() {
        Some(value) => println!("Dequeued: {}", value),
        None => println!("Cannot dequeue: Buffer is empty"),
    }

    // Test adding new elements after empty
    println!("\nTesting adding new elements after empty:");
    buffer.enqueue(42);
    buffer.enqueue(43);
    buffer.display();

    // Test clear operation
    println!("\nTesting clear operation:");
    buffer.clear();
    buffer.display();
}
