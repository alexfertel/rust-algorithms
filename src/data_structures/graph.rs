use std::collections::{hash_map::Entry::Vacant, HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct DirectedGraph<'a, T> {
    adjacency_table: HashMap<&'a T, Vec<(&'a T, i32)>>,
}

impl<'a, T> Graph<'a, T> for DirectedGraph<'a, T>
where
    T: 'a + Eq + Hash,
{
    fn new() -> DirectedGraph<'a, T> {
        DirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>> {
        &self.adjacency_table
    }
}

pub struct UndirectedGraph<'a, T> {
    adjacency_table: HashMap<&'a T, Vec<(&'a T, i32)>>,
}

impl<'a, T> Graph<'a, T> for UndirectedGraph<'a, T>
where
    T: 'a + Eq + Hash,
{
    fn new() -> UndirectedGraph<'a, T> {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&'a T, &'a T, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table.entry(edge.0).and_modify(|e| {
            e.push((edge.1, edge.2));
        });
        self.adjacency_table.entry(edge.1).and_modify(|e| {
            e.push((edge.0, edge.2));
        });
    }
}

pub trait Graph<'a, T>
where
    T: 'a + Eq + Hash,
{
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a T, Vec<(&'a T, i32)>>;
    fn adjacency_table(&self) -> &HashMap<&'a T, Vec<(&'a T, i32)>>;

    fn add_node(&mut self, node: &'a T) -> bool {
        if let Vacant(entry) = self.adjacency_table_mutable().entry(node) {
            entry.insert(Vec::new());
            true
        } else {
            false
        }
    }

    fn add_edge(&mut self, edge: (&'a T, &'a T, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mutable()
            .entry(edge.0)
            .and_modify(|e| {
                e.push((edge.1, edge.2));
            });
    }

    fn neighbours(&self, node: &'a T) -> Result<&Vec<(&'a T, i32)>, NodeNotInGraph> {
        match self.adjacency_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: &'a T) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&'a T> {
        self.adjacency_table().keys().copied().collect()
    }

    fn edges(&self) -> Vec<(&'a T, &'a T, i32)> {
        self.adjacency_table()
            .iter()
            .flat_map(|(from_node, from_node_neighbours)| {
                from_node_neighbours
                    .iter()
                    .map(move |(to_node, weight)| (*from_node, *to_node, *weight))
            })
            .collect()
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph: UndirectedGraph<String> = UndirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&b, &c, 10));
        graph.add_edge((&c, &a, 7));

        let expected_edges = [
            (&a, &b, 5),
            (&b, &a, 5),
            (&c, &a, 7),
            (&a, &c, 7),
            (&b, &c, 10),
            (&c, &b, 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph: UndirectedGraph<String> = UndirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&b, &c, 10));
        graph.add_edge((&c, &a, 7));

        assert_eq!(graph.neighbours(&a).unwrap(), &vec![(&b, 5), (&c, 7)]);
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::DirectedGraph;
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph: DirectedGraph<String> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert_eq!(graph.nodes(), [&a, &b, &c].iter().cloned().collect());
    }

    #[test]
    fn test_add_node_with_struct() {
        #[derive(PartialEq, Eq, Hash, Debug)]
        struct Node {
            name: String,
            value: i32,
        }

        let mut graph: DirectedGraph<Node> = DirectedGraph::new();

        let a = Node {
            name: String::from("a"),
            value: 1,
        };
        let b = Node {
            name: String::from("b"),
            value: 2,
        };
        let c = Node {
            name: String::from("c"),
            value: 3,
        };

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert_eq!(graph.nodes(), [&a, &b, &c].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut graph: DirectedGraph<String> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&c, &a, 7));
        graph.add_edge((&b, &c, 10));

        let expected_edges = [(&a, &b, 5), (&c, &a, 7), (&b, &c, 10)];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph: DirectedGraph<String> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&b, &c, 10));
        graph.add_edge((&c, &a, 7));

        assert_eq!(graph.neighbours(&a).unwrap(), &vec![(&b, 5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph: DirectedGraph<String> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        let d = String::from("d");

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert!(graph.contains(&a));
        assert!(graph.contains(&b));
        assert!(graph.contains(&c));
        assert!(!graph.contains(&d));
    }
}
