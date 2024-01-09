use std::collections::HashMap;

fn find<'a>(set: &mut HashMap<&'a str, &'a str>, vertex: &'a str) -> &'a str {
    if set.get(vertex) == Some(&vertex) {
        vertex
    } else {
        let parent = set.get(vertex).unwrap();
        let root = find(set, parent);
        set.insert(vertex, root);
        root
    }
}

fn kruskal<'a>(
    graph: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    source: &'a str,
) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    let mut mst: HashMap<&'a str, Vec<(&'a str, usize)>> = HashMap::new();

    let mut edges = graph
        .into_iter()
        .flat_map(|(from, tos)| tos.iter().map(move |(to, weight)| (*weight, *from, *to)))
        .collect::<Vec<_>>();
    edges.sort_unstable_by_key(|(weight, _, _)| *weight);

    let mut set = HashMap::new();
    for vertex in graph.keys() {
        set.insert(*vertex, *vertex);
    }

    for (weight, from, to) in edges {
        let root_from = find(&mut set, from);
        let root_to = find(&mut set, to);

        if root_from != root_to {
            mst.entry(from).or_insert_with(Vec::new).push((to, weight));
            mst.entry(to).or_insert_with(Vec::new).push((from, weight));
            set.insert(root_from, root_to);
        }
    }

    mst
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_kruskal() {
        let mut graph = HashMap::new();
        graph.insert("A", vec![("B", 4), ("C", 2)]);
        graph.insert("B", vec![("A", 4), ("C", 3), ("D", 2), ("E", 3)]);
        graph.insert("C", vec![("A", 2), ("B", 3), ("D", 4), ("E", 5)]);
        graph.insert("D", vec![("B", 2), ("C", 4), ("E", 1)]);
        graph.insert("E", vec![("B", 3), ("C", 5), ("D", 1)]);

        let result = kruskal(&graph, "A");

        let answer = {
            let mut mst = HashMap::new();
            mst.insert("A", vec![("C", 2)]);
            mst.insert("B", vec![("D", 2), ("C", 3)]);
            mst.insert("C", vec![("A", 2), ("B", 3)]);
            mst.insert("D", vec![("E", 1), ("B", 2)]);
            mst.insert("E", vec![("D", 1)]);

            mst
        };
        assert_eq!(result, answer);
    }
}
