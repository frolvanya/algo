struct Heap<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: PartialOrd + Copy> Heap<T> {
    fn new() -> Self {
        Heap {
            data: Vec::new(),
            size: 0,
        }
    }

    fn insert(&mut self, val: T) {
        self.data.push(val);
        self.heapify_up(self.size);
        self.size += 1;
    }

    fn delete(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        let out = self.data[0];
        if self.size == 0 {
            return Some(out);
        }

        self.data[0] = self.data[self.size];
        self.heapify_down(0);

        Some(out)
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent = self.parent(index);
        let parent_value = self.data[parent];
        let curr_value = self.data[index];

        if parent_value > curr_value {
            self.data.swap(parent, index);
            self.heapify_up(parent);
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let left_child = self.left_child(index);
        let right_child = self.right_child(index);

        if index >= self.size || left_child >= self.size {
            return;
        }

        let left_value = self.data[left_child];
        let right_value = self.data[right_child];
        let value = self.data[index];

        if left_value > right_value && value > right_value {
            self.data.swap(index, right_child);
            self.heapify_down(right_child);
        } else if right_value > left_value && value > left_value {
            self.data.swap(index, left_child);
            self.heapify_down(left_child);
        }
    }

    fn parent(&self, index: usize) -> usize {
        if index == 0 {
            0
        } else {
            (index - 1) / 2
        }
    }

    fn left_child(&self, index: usize) -> usize {
        index * 2 + 1
    }

    fn right_child(&self, index: usize) -> usize {
        index * 2 + 2
    }
}

#[cfg(test)]
pub mod tests {
    use super::Heap;

    #[test]
    fn test_insert() {
        let mut heap = Heap::new();

        heap.insert(50);
        heap.insert(20);
        heap.insert(30);
        heap.insert(100);
        heap.insert(10);

        assert_eq!(heap.data, vec![10, 20, 30, 100, 50]);
        assert!(heap.size == 5);
    }

    #[test]
    fn test_delete() {
        let mut heap = Heap::new();

        heap.insert(50);
        heap.insert(20);
        heap.insert(30);
        heap.insert(100);
        heap.insert(10);

        assert_eq!(heap.delete(), Some(10));
        assert!(heap.size == 4);

        assert_eq!(heap.delete(), Some(20));
        assert!(heap.size == 3);

        assert_eq!(heap.delete(), Some(30));
        assert!(heap.size == 2);

        assert_eq!(heap.delete(), Some(50));
        assert!(heap.size == 1);

        assert_eq!(heap.delete(), Some(100));
        assert!(heap.size == 0);

        assert_eq!(heap.delete(), None);
    }
}
