use std::collections::{HashSet, VecDeque};

fn topological_sorting_dfs(num_courses: usize, prerequisites: Vec<Vec<usize>>) -> Vec<usize> {
    let mut graph = vec![vec![]; num_courses];
    for p in prerequisites {
        graph[p[0]].push(p[1]);
    }

    fn dfs(
        graph: &Vec<Vec<usize>>,
        course: usize,
        cycle: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
        stack: &mut Vec<usize>,
    ) -> bool {
        if cycle.contains(&course) {
            return false;
        }

        if visited.contains(&course) {
            return true;
        }

        cycle.insert(course);

        for p in &graph[course] {
            if !dfs(graph, *p, cycle, visited, stack) {
                return false;
            }
        }

        cycle.remove(&course);

        visited.insert(course);
        stack.push(course);

        true
    }

    let mut stack = Vec::new();
    let mut cycle = HashSet::new();
    let mut visited = HashSet::new();
    for i in 0..num_courses {
        if !dfs(&graph, i, &mut cycle, &mut visited, &mut stack) {
            return Vec::new();
        }
    }

    stack
}

fn topological_sorting_bfs(num_courses: usize, prerequisites: Vec<Vec<usize>>) -> Vec<usize> {
    let mut graph = vec![vec![]; num_courses];
    let mut in_degree = vec![0; num_courses];

    for p in prerequisites {
        graph[p[1]].push(p[0]);
        in_degree[p[0]] += 1;
    }

    let mut q = VecDeque::new();
    for (i, p) in in_degree.iter().enumerate().take(num_courses) {
        if p == &0 {
            q.push_back(i);
        }
    }

    let mut stack = Vec::new();
    while let Some(course) = q.pop_front() {
        stack.push(course);
        for p in &graph[course] {
            in_degree[*p] -= 1;
            if in_degree[*p] == 0 {
                q.push_back(*p);
            }
        }
    }

    if stack.len() == num_courses {
        stack
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        assert_eq!(topological_sorting_dfs(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            topological_sorting_dfs(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(topological_sorting_dfs(1, vec![]), vec![0]);
    }

    #[test]
    fn test_bfs() {
        assert_eq!(topological_sorting_bfs(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            topological_sorting_bfs(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(topological_sorting_bfs(1, vec![]), vec![0]);
    }
}
