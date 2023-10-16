struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    assert_eq!(stack.length(), 4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.peek(), Some(&3));

    stack.pop();
    stack.pop();
    stack.pop();
    stack.pop();

    assert!(stack.is_empty());
}
