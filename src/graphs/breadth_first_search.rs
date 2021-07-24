use super::representation::{Graph, Vertex};
use std::collections::{HashSet, VecDeque};

pub fn breadth_first_search(graph: &Graph, start: Vertex, end: Vertex) -> bool {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        if !visited.insert(v) {
            continue;
        }

        if v == end {
            return true;
        }

        for neighbor in v.neighbors(graph).into_iter() {
            queue.push_back(neighbor);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_in_kn() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];

        let mut edges = vec![];
        for i in 0..vertices.len() - 1 {
            for j in 1..vertices.len() {
                edges.push((vertices[i], vertices[j]));
                edges.push((vertices[j], vertices[i]));
            }
        }

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(breadth_first_search(&graph, 1.into(), 6.into()));
    }

    #[test]
    fn no_edge() {
        let vertices = vec![1];
        let edges = vec![];

        let graph = Graph::new(vertices.into_iter().map(|v| v.into()).collect(), edges);

        assert!(breadth_first_search(&graph, 1.into(), 1.into()));
    }

    #[test]
    fn directed_right_edge() {
        let vertices = vec![1, 2];
        let edges = vec![(1, 2)];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(breadth_first_search(&graph, 1.into(), 2.into()));
    }

    #[test]
    fn directed_wrong_edge() {
        let vertices = vec![1, 2];
        let edges = vec![(2, 1)];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(!breadth_first_search(&graph, 1.into(), 2.into()));
    }

    #[test]
    fn path() {
        let vertices = vec![1, 2, 3, 4, 5, 6];
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(breadth_first_search(&graph, 1.into(), 6.into()));
    }

    #[test]
    fn divided() {
        let vertices = vec![1, 2, 3, 4, 5, 6];
        let edges = vec![(1, 2), (2, 3), (4, 5), (5, 6)];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(!breadth_first_search(&graph, 1.into(), 6.into()));
    }

    #[test]
    fn island() {
        let vertices = (0..15).into_iter().collect::<Vec<u32>>();
        let edges = vec![
            (0, 1),
            (0, 4),
            (1, 2),
            (2, 3),
            (3, 7),
            (4, 8),
            (8, 9),
            (9, 10),
            (8, 12),
        ];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert!(!breadth_first_search(&graph, 1.into(), 10.into()));
    }
}
