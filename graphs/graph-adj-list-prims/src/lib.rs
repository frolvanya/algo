use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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
        mst.entry(to).or_default().push((from, *weight));

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
        graph.insert("B", vec![("A", 4), ("C", 3), ("D", 2), ("E", 3)]);
        graph.insert("C", vec![("A", 2), ("B", 3), ("D", 4), ("E", 5)]);
        graph.insert("D", vec![("B", 2), ("C", 4), ("E", 1)]);
        graph.insert("E", vec![("B", 3), ("C", 5), ("D", 1)]);

        let result = prims(&graph, "A");

        let answer = {
            let mut mst = HashMap::new();
            mst.insert("A", vec![("C", 2)]);
            mst.insert("B", vec![("C", 3), ("D", 2)]);
            mst.insert("C", vec![("A", 2), ("B", 3)]);
            mst.insert("D", vec![("B", 2), ("E", 1)]);
            mst.insert("E", vec![("D", 1)]);

            mst
        };
        assert_eq!(result, answer);
    }
}
