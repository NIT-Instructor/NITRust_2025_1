struct FifoBuffer {
    buffer: Vec<u8>,
    head: usize,
    tail: usize,
    size: usize,
}

impl FifoBuffer {
    fn new(capacity: usize) -> FifoBuffer {
        FifoBuffer {
            buffer: vec![0; capacity],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn enqueue(&mut self, val: u8) -> Result<(), &str> {
        if self.size == self.buffer.len() {
            Err("Buffer is full")
        } else {
            self.buffer[self.tail] = val;
            self.tail += 1;
            if self.tail == self.buffer.len() {
                self.tail = 0;
            }
            self.size += 1;
            Ok(())
        }
    }

    fn dequeue(&mut self) -> Option<u8> {
        if self.size == 0 {
            None
        } else {
            let val = self.buffer[self.head];
            self.head += 1;
            if self.head == self.buffer.len() {
                self.head = 0;
            }
            self.size -= 1;
            Some(val)
        }
    }
}

#[test]
fn fifo_buffer_test() {
    let mut buffer = FifoBuffer::new(3);

    buffer.enqueue(1).unwrap();
    buffer.enqueue(2).unwrap();
    buffer.enqueue(3).unwrap();
    assert_eq!(buffer.enqueue(4), Err("Buffer is full"));

    assert_eq!(buffer.dequeue(), Some(1));
    buffer.enqueue(4).unwrap();

    assert_eq!(buffer.dequeue(), Some(2));
    assert_eq!(buffer.dequeue(), Some(3));
    assert_eq!(buffer.dequeue(), Some(4));
    assert_eq!(buffer.dequeue(), None);
}
