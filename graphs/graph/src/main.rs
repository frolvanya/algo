use std::collections::{HashMap, VecDeque};

struct Graph {
    vertices: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: i32) {
        self.vertices.entry(vertex).or_insert(Vec::new());
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.vertices.entry(u).or_insert(Vec::new()).push(v);
        self.vertices.entry(v).or_insert(Vec::new()).push(u);
    }

    fn get_neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.vertices.get(&vertex)
    }

    fn dfs_rec(
        &self,
        current_vertex: i32,
        target_vertex: i32,
        mut visited: HashMap<i32, bool>,
        mut path: Vec<i32>,
    ) -> Option<Vec<i32>> {
        visited.insert(current_vertex, true);
        path.push(current_vertex);

        if current_vertex == target_vertex {
            return Some(path);
        }

        if let Some(neighbors) = self.get_neighbors(current_vertex) {
            for neighbor in neighbors {
                if !visited.get(neighbor).unwrap_or(&false) {
                    if let Some(res) =
                        self.dfs_rec(*neighbor, target_vertex, visited.clone(), path.clone())
                    {
                        return Some(res);
                    }
                }
            }
        }

        None
    }

    fn dfs(&self, mut current_vertex: i32, target_vertex: i32) -> Option<Vec<i32>> {
        let mut stack = Vec::new();
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let mut parent = HashMap::new();

        stack.push(current_vertex);

        while !stack.is_empty() {
            current_vertex = stack.pop().unwrap();

            if current_vertex == target_vertex {
                println!("{:?}", parent);
                let mut result = vec![current_vertex];
                let mut node = current_vertex;
                while let Some(&p) = parent.get(&node) {
                    result.push(p);
                    node = p;
                }
                result.reverse();

                return Some(result);
            }

            if !visited.get(&current_vertex).unwrap_or(&false) {
                if let Some(neighbors) = self.get_neighbors(current_vertex) {
                    visited.insert(current_vertex, true);
                    path.push(current_vertex);

                    for neighbor in neighbors {
                        if !visited.get(neighbor).unwrap_or(&false) {
                            stack.push(*neighbor);
                            parent.insert(*neighbor, current_vertex);
                        }
                    }
                }
            } else {
                while let Some(last) = path.pop() {
                    if last == current_vertex {
                        break;
                    }
                }
            }
        }

        None
    }

    fn bfs(&self, mut current_vertex: i32, target_vertex: i32) -> Option<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut path = Vec::new();

        queue.push_back(current_vertex);

        while !queue.is_empty() {
            current_vertex = queue.pop_front().unwrap();
            if current_vertex == target_vertex {
                return Some(path);
            }

            if let Some(neighbors) = self.get_neighbors(current_vertex) {
                for neighbor in neighbors {
                    if !visited.get(neighbor).unwrap_or(&false) {
                        visited.insert(*neighbor, true);
                        queue.push_back(*neighbor);
                        path.push(*neighbor);
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new();

    for i in 0..10 {
        graph.add_vertex(i);
    }

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 5);
    graph.add_edge(2, 6);
    graph.add_edge(3, 7);
    graph.add_edge(4, 7);
    graph.add_edge(5, 8);
    graph.add_edge(6, 8);
    graph.add_edge(7, 9);
    graph.add_edge(8, 9);

    println!("{:?}", graph.dfs(0, 9));
    println!("{:?}", graph.dfs_rec(0, 9, HashMap::new(), Vec::new()));
    println!("{:?}", graph.bfs(0, 9));
}
