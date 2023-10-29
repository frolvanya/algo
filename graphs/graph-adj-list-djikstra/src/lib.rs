use std::{cmp::Reverse, collections::BinaryHeap};

fn djikstra(graph: Vec<Vec<(i32, usize)>>, source: i32, target: i32) -> Vec<i32> {
    let mut dists = vec![usize::MAX; graph.len()];
    dists[source as usize] = 0;

    let mut prev = vec![-1; graph.len()];

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), source));

    while let Some((dist, edge)) = pq.pop() {
        let adjs = graph[edge as usize].clone();

        for (adj, weight) in adjs {
            if dist.0 + weight < dists[adj as usize] {
                dists[adj as usize] = dist.0 + weight;
                pq.push((Reverse(dist.0 + weight), adj));
                prev[adj as usize] = edge;
            }
        }
    }

    let mut path = Vec::new();
    let mut curr = target;

    while prev[curr as usize] != -1 {
        path.push(curr);
        curr = prev[curr as usize];
    }

    path.push(source);
    path.reverse();

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_djikstra() {
        let graph = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![],
        ];

        let path = djikstra(graph, 0, 3);
        assert_eq!(path, vec![0, 2, 1, 3]);
    }
}
