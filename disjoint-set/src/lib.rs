use std::cmp::Ordering;

#[derive(Debug)]
struct DisjoinSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjoinSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (x_parent, y_parent) = (self.find(x), self.find(y));

        if x_parent == y_parent {
            return false;
        }

        match self.rank[x_parent].cmp(&self.rank[y_parent]) {
            Ordering::Less => self.parent[x_parent] = y_parent,
            Ordering::Greater => self.parent[y_parent] = x_parent,
            Ordering::Equal => {
                self.parent[y_parent] = x_parent;
                self.rank[x_parent] += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint() {
        let mut disjoint = DisjoinSet::new(5);
        assert_eq!(disjoint.find(0), 0);
        assert_eq!(disjoint.find(1), 1);
        assert_eq!(disjoint.find(2), 2);
        assert_eq!(disjoint.find(3), 3);
        assert_eq!(disjoint.find(4), 4);

        disjoint.union(0, 1);
        assert_eq!(disjoint.find(1), 0);

        disjoint.union(0, 2);
        assert_eq!(disjoint.find(2), 0);
    }
}
