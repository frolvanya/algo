struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        Some(self.queue.remove(0))
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);

    assert_eq!(queue.length(), 2);
    assert_eq!(queue.peek(), Some(&1));
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}
