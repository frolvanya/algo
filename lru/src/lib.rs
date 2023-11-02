use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    next: Link<K, V>,
    prev: Link<K, V>,
}

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[allow(dead_code)]
impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            next: None,
            prev: None,
        }
    }
}

impl<K, V> std::fmt::Debug for Node<K, V>
where
    K: std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut next = None;
        let mut prev = None;
        if let Some(node) = &self.next {
            next = Some(node.borrow().val.clone());
        }
        if let Some(node) = &self.prev {
            prev = Some(node.borrow().val.clone());
        }
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("val", &self.val)
            .field("next", &next)
            .field("prev", &prev)
            .finish()
    }
}

#[allow(dead_code)]
struct Lru<K, V> {
    length: usize,
    capacity: usize,

    head: Link<K, V>,
    tail: Link<K, V>,

    lookup: HashMap<K, Link<K, V>>,
}

#[allow(dead_code)]
impl<K, V> Lru<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: PartialEq + Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity,
            head: None,
            tail: None,
            lookup: HashMap::with_capacity(capacity + 1),
        }
    }

    fn update(&mut self, key: K, value: V) {
        if let Some(mut node) = self.lookup.get(&key).cloned() {
            self.detach(&node);
            self.prepend(&mut node);

            if let Some(inside_node) = node {
                inside_node.borrow_mut().val = value;
            }
        } else {
            let node = Rc::new(RefCell::new(Node::new(key.clone(), value)));

            self.length += 1;
            self.prepend(&mut Some(node.clone()));
            self.trim_cache();

            self.lookup.insert(key, Some(node));
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        let mut node = match self.lookup.get(&key).cloned() {
            Some(node) => node,
            None => return None,
        };

        if let Some(inside_node) = node.clone() {
            self.detach(&node);
            self.prepend(&mut node);

            return Some(inside_node.borrow().val.clone());
        }

        None
    }

    fn trim_cache(&mut self) {
        if self.length > self.capacity {
            if let Some(inside_tail) = self.tail.clone() {
                self.detach(&self.tail.clone());

                let key = inside_tail.borrow().key.clone();

                self.lookup.remove(&key);
                self.length -= 1;
            }
        }
    }

    fn detach(&mut self, node: &Link<K, V>) {
        if let Some(inside_node) = &node {
            if let Some(prev_node) = &inside_node.borrow().prev {
                prev_node.borrow_mut().next = inside_node.borrow().next.clone();
            }

            if let Some(next_node) = &inside_node.borrow().next {
                next_node.borrow_mut().prev = inside_node.borrow().prev.clone();
            }

            if &self.head == node {
                if let Some(inside_head) = self.head.take() {
                    self.head = inside_head.borrow().next.clone();
                }
            }

            if &self.tail == node {
                if let Some(inside_tail) = self.tail.take() {
                    self.tail = inside_tail.borrow().prev.clone();
                }
            }

            inside_node.borrow_mut().prev = None;
            inside_node.borrow_mut().next = None;
        }
    }

    fn prepend(&mut self, node: &mut Link<K, V>) {
        if let Some(inside_head) = &self.head {
            if let Some(inside_node) = node {
                inside_node.borrow_mut().next = self.head.clone();
            }

            inside_head.borrow_mut().prev = node.clone();
        } else {
            self.tail = node.clone();
        }

        self.head = node.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru() {
        let mut lru = Lru::new(3);

        assert_eq!(lru.get("foo"), None);
        lru.update("foo", 69);
        assert_eq!(lru.get("foo"), Some(69));

        lru.update("bar", 420);
        assert_eq!(lru.get("bar"), Some(420));

        lru.update("baz", 1337);
        assert_eq!(lru.get("baz"), Some(1337));

        lru.update("ball", 69420);
        assert_eq!(lru.get("ball"), Some(69420));
        assert_eq!(lru.get("foo"), None);
        assert_eq!(lru.get("bar"), Some(420));

        lru.update("foo", 69);
        assert_eq!(lru.get("bar"), Some(420));
        assert_eq!(lru.get("foo"), Some(69));

        assert_eq!(lru.get("baz"), None);
    }
}
