#[derive(Clone)]
enum Cell {
    Empty,
    Used(i32),
    Deleted,
}

struct HashTable {
    cells: Vec<Cell>,
    capacity: usize,
}

impl HashTable {
    fn new(capacity: usize) -> Self {
        let cells = vec![Cell::Empty; capacity];
        HashTable { cells, capacity }
    }

    const fn hash(&self, key: i32) -> usize {
        (key as usize) % self.capacity
    }

    fn insert(&mut self, key: i32) {
        let mut index = self.hash(key);

        while match self.cells.get(index) {
            Some(Cell::Used(k)) => k != &key,
            _ => false,
        } {
            index = (index + 1) % self.capacity;
        }

        if let Some(cell) = self.cells.get_mut(index) {
            *cell = Cell::Used(key);
        }
    }

    fn search(&self, key: i32) -> bool {
        let mut index = self.hash(key);

        while match self.cells.get(index) {
            Some(Cell::Used(k)) => k != &key,
            Some(Cell::Deleted) => true,
            _ => false,
        } {
            index = (index + 1) % self.capacity;
        }

        match self.cells.get(index) {
            Some(Cell::Used(k)) => k == &key,
            _ => false,
        }
    }

    fn delete(&mut self, key: i32) {
        let mut index = self.hash(key);

        while match self.cells.get(index) {
            Some(Cell::Used(k)) => k != &key,
            Some(Cell::Deleted) => true,
            _ => false,
        } {
            index = (index + 1) % self.capacity;
        }

        if let Some(cell) = self.cells.get_mut(index) {
            *cell = Cell::Deleted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tombstoning() {
        let mut ht = HashTable::new(10);

        ht.insert(0);
        ht.insert(1);
        ht.insert(2);
        ht.insert(3);
        ht.insert(10);
        ht.insert(20);
        ht.insert(21);

        assert!(ht.search(0));
        assert!(ht.search(1));
        assert!(ht.search(2));
        assert!(ht.search(3));
        assert!(ht.search(10));
        assert!(ht.search(20));
        assert!(ht.search(21));
        assert!(!ht.search(22));

        ht.delete(0);
        assert!(!ht.search(0));
        assert!(ht.search(10));
        assert!(ht.search(20));

        ht.delete(1);
        assert!(!ht.search(1));
        assert!(ht.search(21));
    }
}
