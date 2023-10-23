use std::collections::VecDeque;

fn bfs(graph: Vec<Vec<i32>>, source: i32, needle: i32) -> Option<Vec<i32>> {
    let mut seen = vec![false; graph.len()];
    let mut prev = vec![-1; graph.len()];

    seen[source as usize] = true;

    let mut q = VecDeque::new();
    q.push_back(source);

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        if curr == needle {
            break;
        }

        let adjs = graph[curr as usize].clone();
        for i in 0..adjs.len() {
            if adjs[i] == 0 || seen[i] {
                continue;
            }

            seen[i] = true;
            prev[i] = curr;
            q.push_back(i as i32);
        }
        seen[curr as usize] = true;
    }

    let mut curr = needle as usize;
    let mut path = Vec::new();

    dbg!(&prev);

    while prev[curr] != -1 {
        path.push(curr as i32);
        curr = prev[curr] as usize;
    }

    if path.is_empty() {
        return None;
    }

    path.push(source);
    Some(path.into_iter().rev().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let matrix = vec![
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
        ];

        let path = bfs(matrix.clone(), 0, 6);
        assert_eq!(path, Some(vec![0, 3, 4, 6]));

        let path = bfs(matrix, 0, 1);
        assert_eq!(path, None);
    }
}
