use crate::data_structures::{Graph, UndirectedGraph};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub fn color_graph<'a, T>(graph: &'a UndirectedGraph<T>) -> HashMap<&'a T, u32>
where
    T: 'a + Eq + Hash + Debug,
{
    let empty_vec = Vec::new();
    graph
        .nodes()
        .iter()
        .fold(HashMap::new(), |mut colors, node| {
            let used_colors = graph
                .neighbours(node)
                .unwrap_or(&empty_vec)
                .iter()
                .filter_map(|(neighbor, _)| colors.get(neighbor))
                .collect::<HashSet<_>>();

            let color = (0..).find(|i| !used_colors.contains(i)).unwrap_or(0);
            colors.insert(node, color);

            colors
        })
}

#[cfg(test)]
mod test_color_graph {
    use super::*;

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Node {
        name: String,
    }

    impl Node {
        fn new(name: String) -> Self {
            Node { name }
        }
    }

    #[test]
    fn test_coloring() {
        // a->b
        // b->c
        // c->a
        // a->d
        // a->e

        // ┌───────┐
        // │   a   │
        // └△─┬─┬─┬┘
        //  │ │ │┌▽┐
        //  │ │ ││b│
        //  │ │ │└┬┘
        //  │ │┌▽┐│
        //  │ ││d││
        //  │ │└─┘│
        //  │┌▽┐  │
        //  ││e│  │
        //  │└─┘  │
        // ┌┴─────▽─┐
        // │   c    │
        // └────────┘

        let mut graph: UndirectedGraph<Node> = UndirectedGraph::new();

        let a = Node::new(String::from("a"));
        let b = Node::new(String::from("b"));
        let c = Node::new(String::from("c"));
        let d = Node::new(String::from("d"));
        let e = Node::new(String::from("e"));

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);
        graph.add_node(&d);
        graph.add_node(&e);

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&b, &c, 10));
        graph.add_edge((&c, &a, 7));
        graph.add_edge((&a, &d, 5));
        graph.add_edge((&a, &e, 5));

        let colors = color_graph(&graph);

        assert_ne!(colors[&a], colors[&b]);
        assert_ne!(colors[&b], colors[&c]);
        assert_ne!(colors[&c], colors[&a]);
        assert_ne!(colors[&a], colors[&d]);
        assert_ne!(colors[&a], colors[&e]);

        let all_colors = vec![0, 1, 2];
        assert!(all_colors.contains(&colors[&a]));
        assert!(all_colors.contains(&colors[&b]));
        assert!(all_colors.contains(&colors[&c]));
        assert!(all_colors.contains(&colors[&d]));
        assert!(all_colors.contains(&colors[&e]));
    }

    #[test]
    fn test_coloring_each_to_each() {
        // a->b
        // a->c
        // a->d
        // a->e
        // b->c
        // b->d
        // b->e
        // c->d
        // c->e
        // d->e

        // ┌──────┐
        // │a     │
        // └┬┬─┬─┬┘
        //  ││ │┌▽────┐
        //  ││ ││b    │
        //  ││ │└┬─┬┬─┘
        //  ││┌▽─▽┐││
        //  │││c  │││
        //  ││└─┬┬┘││
        //  ││  ││ ││
        //  ││  ││ ││
        //  ││  ││ ││
        //  │└─┐││ ││
        //  │┌─│┘│ ││
        //  ││ │┌│─│┘
        //  ││┌▽▽▽┐│
        //  │││d  ││
        //  ││└┬──┘│
        // ┌▽▽─▽───▽┐
        // │e       │
        // └────────┘

        let mut graph: UndirectedGraph<Node> = UndirectedGraph::new();

        let a = Node::new(String::from("a"));
        let b = Node::new(String::from("b"));
        let c = Node::new(String::from("c"));
        let d = Node::new(String::from("d"));
        let e = Node::new(String::from("e"));

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);
        graph.add_node(&d);
        graph.add_node(&e);

        graph.add_edge((&a, &b, 5));
        graph.add_edge((&a, &c, 5));
        graph.add_edge((&a, &d, 5));
        graph.add_edge((&a, &e, 5));
        graph.add_edge((&b, &c, 5));
        graph.add_edge((&b, &d, 5));
        graph.add_edge((&b, &e, 5));
        graph.add_edge((&c, &d, 5));
        graph.add_edge((&c, &e, 5));
        graph.add_edge((&d, &e, 5));

        let colors = color_graph(&graph);

        assert_ne!(colors[&a], colors[&b]);
        assert_ne!(colors[&b], colors[&c]);
        assert_ne!(colors[&c], colors[&d]);
        assert_ne!(colors[&d], colors[&e]);
        assert_ne!(colors[&e], colors[&a]);

        let all_colors = vec![0, 1, 2, 3, 4, 5];
        assert!(all_colors.contains(&colors[&a]));
        assert!(all_colors.contains(&colors[&b]));
        assert!(all_colors.contains(&colors[&c]));
        assert!(all_colors.contains(&colors[&d]));
        assert!(all_colors.contains(&colors[&e]));
    }
}
