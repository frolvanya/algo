use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn djikstra<'a>(
    graph: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    source: &'a str,
    target: &'a str,
) -> Vec<&'a str> {
    let mut dists = HashMap::new();
    dists.insert(source, 0);

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), source));

    let mut prev = HashMap::<&str, &str>::new();
    while let Some((weight, from)) = pq.pop() {
        if from == target {
            let mut path = Vec::new();
            let mut curr = target;

            while let Some(prev) = prev.get(curr) {
                path.push(curr);
                curr = prev;
            }

            path.push(source);
            path.reverse();

            return path;
        }

        let neighbors = &graph[from];

        for (to, dist) in neighbors {
            if weight.0 + dist < *dists.entry(to).or_insert(usize::MAX) {
                dists.insert(to, weight.0 + dist);
                prev.insert(to, from);
                pq.push((Reverse(weight.0 + dist), to));
            }
        }
    }

    Vec::new()
}

fn prims<'a>(
    graph: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    source: &'a str,
) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    let mut mst = HashMap::new();
    mst.insert(source, Vec::new());

    let mut pq = BinaryHeap::new();
    for (to, weight) in &graph[source] {
        pq.push((Reverse(weight), source, to));
    }

    while let Some((Reverse(weight), from, to)) = pq.pop() {
        if mst.contains_key(to) {
            continue;
        }

        mst.entry(from).or_default().push((*to, *weight));
        mst.insert(to, Vec::new());

        for (edge, weight) in &graph[to] {
            pq.push((Reverse(weight), to, edge));
        }
    }

    mst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prims() {
        let mut graph = HashMap::new();
        graph.insert("A", vec![("B", 4), ("C", 2)]);
        graph.insert("B", vec![("C", 3), ("D", 2), ("E", 3)]);
        graph.insert("C", vec![("B", 1), ("D", 4), ("E", 5)]);
        graph.insert("D", vec![]);
        graph.insert("E", vec![("D", 1)]);

        let result = prims(&graph, "A");

        let answer = {
            let mut mst = HashMap::new();
            mst.insert("A", vec![("C", 2)]);
            mst.insert("C", vec![("B", 1)]);
            mst.insert("B", vec![("D", 2), ("E", 3)]);
            mst.insert("D", vec![]);
            mst.insert("E", vec![]);

            mst
        };
        assert_eq!(result, answer);
    }
}
