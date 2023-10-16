struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Eq + core::fmt::Display + Clone> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }

    fn append(&mut self, val: T) {
        match self.next {
            Some(ref mut next) => next.append(val),
            None => self.next = Some(Box::new(Node::new(val))),
        }
    }

    fn delete(&mut self, element: T) {
        if let Some(ref mut next) = self.next {
            if next.val == element {
                self.next = next.next.take();
            } else {
                next.delete(element);
            }
        }
    }

    fn update(&mut self, index: usize, element: T) {
        let mut i = 0;
        let mut current = self;

        if i == index {
            current.val = element.clone();
        }

        while i < index {
            if let Some(ref mut next) = current.next {
                current = next;
            }

            i += 1;
        }

        current.val = element;
    }

    fn length(&self) -> usize {
        match self.next {
            Some(ref next) => 1 + next.length(),
            None => 1,
        }
    }

    fn list(&self) {
        if let Some(ref next) = self.next {
            print!("{} -> ", self.val);
            next.list()
        } else {
            println!("{}", self.val);
        }
    }
}

fn main() {
    let mut head = Node::new(1);

    head.append(2);
    head.append(3);
    head.append(4);
    head.append(5);

    head.list();
    println!("length: {}", head.length());

    head.delete(4);

    head.list();
    println!("length: {}", head.length());

    head.delete(5);
    head.list();
    println!("length: {}", head.length());

    // head.delete(1);
    // head.list();
    // println!("length: {}", head.length());
    // Another way to delete head node
    head = *head.next.unwrap();
    head.list();
    println!("length: {}", head.length());

    head.update(0, 10);
    head.list();
    println!("length: {}", head.length());

    head.update(1, 20);
    head.list();
    println!("length: {}", head.length());
}
