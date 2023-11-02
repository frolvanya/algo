use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Graph {
    vertices: HashMap<i32, Vec<(i32, i32)>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: i32) {
        self.vertices.entry(vertex).or_default();
    }

    fn add_edge(&mut self, u: i32, v: i32, weight: i32) {
        self.vertices.entry(u).or_default().push((v, weight));
        self.vertices.entry(v).or_default().push((u, weight));
    }

    fn get_neighbors(&self, vertex: i32) -> Option<&Vec<(i32, i32)>> {
        self.vertices.get(&vertex)
    }

    fn djikstra(&self, start: i32, target: i32) -> Option<(i32, Vec<i32>)> {
        let mut distances = vec![std::i32::MAX; self.vertices.len()];
        let mut previous_vertices = vec![None; self.vertices.len()];
        let mut pq = BinaryHeap::new();

        distances[start as usize] = 0;
        pq.push((Reverse(0), start));

        while let Some((Reverse(dist), current_vertex)) = pq.pop() {
            if current_vertex == target {
                let mut path = vec![];
                let mut current = current_vertex;
                while let Some(prev) = previous_vertices[current as usize] {
                    path.push(current);
                    current = prev;
                }
                path.push(start);
                path.reverse();
                return Some((dist, path));
            }

            if let Some(neighbors) = self.get_neighbors(current_vertex) {
                for (neighbor, weight) in neighbors {
                    if dist + weight < distances[*neighbor as usize] {
                        distances[*neighbor as usize] = dist + weight;
                        previous_vertices[*neighbor as usize] = Some(current_vertex);
                        pq.push((Reverse(dist + weight), *neighbor));
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_vertex(0);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);

    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 3);
    graph.add_edge(0, 2, 5);
    graph.add_edge(2, 3, 1);

    println!("{:?}", graph.get_neighbors(0));
    println!("{:?}", graph.djikstra(0, 3));
}
