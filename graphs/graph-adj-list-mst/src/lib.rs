use std::collections::{HashMap, HashSet};

fn prims<'a>(graph: &HashMap<&'a str, Vec<(&'a str, usize)>>, source: &'a str) -> HashSet<String> {
    let mut mst = HashSet::new();

    mst.insert(source.to_string());

    let mut edges = graph.get(source).unwrap().clone();
    while mst.len() < graph.len() {
        let mut min_edge = None;
        let mut min_weight = usize::MAX;

        for (edge, weight) in edges.clone() {
            if !mst.contains(edge) && weight < min_weight {
                min_edge = Some(edge);
                min_weight = weight;
            }
        }

        if let Some(min_edge_value) = min_edge {
            mst.insert(min_edge_value.to_string());

            for &edge in graph.get(min_edge_value).unwrap() {
                edges.push(edge);
            }

            edges.retain(|edge| edge.0 != min_edge_value);
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

        let answer = HashSet::from_iter(
            vec!["A", "B", "C", "D", "E"]
                .into_iter()
                .map(|x| x.to_string()),
        );
        assert_eq!(result, answer);
    }
}
