use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: Link::None,
            prev: Link::None,
        }
    }
}

struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T: Copy + PartialEq> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: Link::None,
            tail: Link::None,
            length: 0,
        }
    }

    fn prepend(&mut self, val: T) {
        self.length += 1;

        let node = Rc::new(RefCell::new(Node::new(val)));

        if let Some(inside_head) = self.head.take() {
            node.borrow_mut().next = Some(inside_head.clone());
            inside_head.borrow_mut().prev = Some(node.clone());
        } else {
            self.tail = Some(node.clone());
        }

        self.head = Some(node);
    }

    fn append(&mut self, val: T) {
        self.length += 1;

        let node = Rc::new(RefCell::new(Node::new(val)));

        if let Some(inside_tail) = self.tail.take() {
            inside_tail.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(inside_tail.clone());
        } else {
            self.head = Some(node.clone());
        }

        self.tail = Some(node);
    }

    fn insert_at(&mut self, index: usize, val: T) -> Result<(), Box<dyn std::error::Error>> {
        if index > self.length {
            return Err("Index out of bounds".into());
        } else if index == self.length {
            return {
                self.append(val);
                Ok(())
            };
        } else if index == 0 {
            return {
                self.prepend(val);
                Ok(())
            };
        }

        self.length += 1;

        let curr = self.get_at(index);

        let node = Rc::new(RefCell::new(Node::new(val)));

        if let Some(inside_curr) = curr.clone() {
            node.borrow_mut().prev = inside_curr.borrow().prev.clone();
            node.borrow_mut().next = curr.clone();

            if let Some(prev_curr) = inside_curr.borrow().prev.clone() {
                prev_curr.borrow_mut().next = Some(node.clone());
            }

            inside_curr.borrow_mut().prev = Some(node.clone());
        }

        Ok(())
    }

    fn remove(&mut self, val: T) -> Result<(), Box<dyn std::error::Error>> {
        let mut curr = self.head.clone();

        for _ in 0..self.length {
            if let Some(inside_curr) = curr.clone() {
                if inside_curr.borrow().val == val {
                    return self.remove_node(curr);
                }

                if let Some(next_curr) = inside_curr.borrow().next.clone() {
                    curr = Some(next_curr);
                } else {
                    return Err("Value not found".into());
                }
            }
        }

        Ok(())
    }

    fn remove_at(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let curr = self.get_at(index);

        if let Some(inside_curr) = curr {
            return self.remove_node(Some(inside_curr));
        }

        Err("Index out of bounds".into())
    }

    fn remove_node(&mut self, node: Link<T>) -> Result<(), Box<dyn std::error::Error>> {
        self.length -= 1;

        if self.length == 0 {
            self.head = None;
            self.tail = None;

            return Ok(());
        }

        if let Some(inside_node) = node {
            if let Some(prev_node) = &inside_node.borrow().prev {
                prev_node.borrow_mut().next = inside_node.borrow().next.clone();
            }

            if let Some(next_node) = &inside_node.borrow().next {
                next_node.borrow_mut().prev = inside_node.borrow().prev.clone();
            }

            if self.head == Some(inside_node.clone()) {
                self.head = inside_node.borrow().next.clone();
            }

            if self.tail == Some(inside_node.clone()) {
                self.tail = inside_node.borrow().prev.clone();
            }

            inside_node.borrow_mut().prev = None;
            inside_node.borrow_mut().next = None;
        }

        Ok(())
    }

    fn get(&self, index: usize) -> Option<T> {
        self.get_at(index).map(|node| node.borrow().val)
    }

    fn get_at(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if index >= self.length {
            return None;
        }

        let mut curr = self.head.clone();

        for _ in 0..index {
            if let Some(inside_curr) = curr.clone() {
                curr = inside_curr.borrow().next.clone();
            } else {
                return None;
            }
        }

        curr
    }
}

impl<T: Copy + std::fmt::Debug> std::fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();

        let mut curr = self.head.clone();

        for _ in 0..self.length {
            if let Some(inside_curr) = curr.clone() {
                result.push(inside_curr.borrow().val);
                curr = inside_curr.borrow().next.clone();
            } else {
                break;
            }
        }

        write!(f, "{:?}", result)
    }
}

impl<T: Copy> Iterator for DoublyLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.borrow().next.clone();
            node.borrow().val
        })
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);
    list.append(6);

    list.prepend(0);

    list.remove_at(1).unwrap();
    list.remove_at(0).unwrap();
    list.remove_at(2).unwrap();

    list.insert_at(0, 10).unwrap();
    list.insert_at(0, 20).unwrap();
    list.insert_at(1, 30).unwrap();

    list.remove(30).unwrap();

    println!("{:?}", list.get(0));
    println!("{:?}", list);

    for element in list {
        print!("{} -> ", element);
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);
        list.prepend(0);

        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(1), Some(1));
        assert_eq!(list.get(2), Some(2));
        assert_eq!(list.get(3), Some(3));
    }

    #[test]
    fn test_insert() {
        let mut list = DoublyLinkedList::new();

        assert!(list.insert_at(1, 0).is_err());

        assert!(list.insert_at(0, 1).is_ok());
        assert_eq!(list.get(0), Some(1));

        assert!(list.insert_at(1, 3).is_ok());
        assert_eq!(list.get(1), Some(3));

        assert!(list.insert_at(1, 2).is_ok());
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
    }

    #[test]
    fn test_remove() {
        let mut list = DoublyLinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);

        assert!(list.remove(4).is_err());

        assert!(list.remove(1).is_ok());
        assert_eq!(list.get(0), Some(2));

        assert!(list.remove(2).is_ok());
        assert_eq!(list.get(0), Some(3));
    }

    #[test]
    fn test_remove_at() {
        let mut list = DoublyLinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);

        assert!(list.remove_at(3).is_err());

        assert!(list.remove_at(1).is_ok());
        assert_eq!(list.get(1), Some(3));

        assert!(list.remove_at(0).is_ok());
        assert_eq!(list.get(0), Some(3));
    }

    #[test]
    fn test_get() {
        let mut list = DoublyLinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));

        assert!(list.get(3).is_none());
    }
}
