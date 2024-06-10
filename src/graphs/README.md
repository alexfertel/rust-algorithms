## General

### [Bellman Ford](./bellman_ford.rs)
![alt text][ford]

The Bellman–Ford algorithm is an algorithm that computes shortest paths from a single source vertex to all of the other vertices in a weighted digraph.[1] It is slower than Dijkstra's algorithm for the same problem, but more versatile, as it is capable of handling graphs in which some of the edge weights are negative numbers.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm)

### [Breadth First Search](./breadth_first_search.rs)
![alt text][search]

Breadth-first search (BFS) is an algorithm for searching a tree data structure for a node that satisfies a given property. It starts at the tree root and explores all nodes at the present depth prior to moving on to the nodes at the next depth level. Extra memory, usually a queue, is needed to keep track of the child nodes that were encountered but not yet explored.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Breadth-first_search)

### [Centroid Decomposition](./centroid_decomposition.rs)
![alt text][centroid]

Centroid of a Tree is a node which if removed from the tree would split it into a ‘forest’, such that any tree in the forest would have at most half the number of vertices in the original tree.<br>

###### Source: [Geeks for Geeks](https://www.geeksforgeeks.org/centroid-decomposition-of-tree/)

### [Depth First Search](./depth_first_search.rs)
![alt text][1search]

Depth-first search (DFS) is an algorithm for traversing or searching tree or graph data structures. The algorithm starts at the root node (selecting some arbitrary node as the root node in the case of a graph) and explores as far as possible along each branch before backtracking. Extra memory, usually a stack, is needed to keep track of the nodes discovered so far along a specified branch which helps in backtracking of the graph.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Depth-first_search)

### [Depth First Search Tic Tac Toe Implementation](./depth_first_search_tic_tac_toe.rs)
![alt text][tic]

This can be used to solve a game, to find the best possible move or simply who wins given ideal gameplay. This form of game AI is amongst the easiest to implement, since it doesn’t require the construction of a tree. Since this algorithm works bottom up without rechecking any nodes, it is typically using a recursive function and a function that checks if the game is over.

For the game of Tic Tac Toe, consider the following method<br>

###### Source: [Blog](https://blog.theofekfoundation.org/artificial-intelligence/2015/12/10/tic-tac-toe-ai-with-depth-first-search/)

### [Dijkstra](./dijkstra.rs)
![alt text][Dijkstra]

Dijkstra's algorithm is an algorithm for finding the shortest paths between nodes in a weighted graph, which may represent, for example, road networks.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)

### [Dinic's Maxflow](./dinic_maxflow.rs)
![alt text][maxflow]

Dinic's algorithm or Dinitz's algorithm is a strongly polynomial algorithm for computing the maximum flow in a flow network, conceived in 1970 by Israeli (formerly Soviet) computer scientist Yefim Dinitz. The introduction of the concepts of the level graph and blocking flow enable Dinic's algorithm to achieve its performance.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Dinic%27s_algorithm)

### [Disjoint Set Union](./disjoint_set_union.rs)
![alt text][set]

Two sets are called disjoint sets if they don’t have any element in common, the intersection of sets is a null set. Consider a situation with a number of persons and the following tasks to be performed on them:

Add a new friendship relation, i.e. a person x becomes the friend of another person y i.e adding new element to a set.
Find whether individual x is a friend of individual y (direct or indirect friend). Find whether x and y belong to the same group or not, i.e. to find if x and y are direct/indirect friends.<br>

###### Source: [Geeks for Geeks](https://www.geeksforgeeks.org/introduction-to-disjoint-set-data-structure-or-union-find-algorithm/)

### [Graph Enumeration](./graph_enumeration.rs)
![alt text][graph]

In combinatorics, an area of mathematics, graph enumeration describes a class of combinatorial enumeration problems in which one must count undirected or directed graphs of certain types, typically as a function of the number of vertices of the graph. These problems may be solved either exactly (as an algebraic enumeration problem) or asymptotically. The pioneers in this area of mathematics were George Pólya, Arthur Cayley and J. Howard Redfield.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Graph_enumeration)

### [Heavy Light Decomposition](./heavy_light_decomposition.rs)
![alt text][light]

In combinatorial mathematics and theoretical computer science, heavy-light decomposition (also called heavy path decomposition) is a technique for decomposing a rooted tree into a set of paths. In a heavy path decomposition, each non-leaf node selects one "heavy edge", the edge to the child that has the greatest number of descendants (breaking ties arbitrarily). The selected edges form the paths of the decomposition.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Heavy-light_decomposition)

### [Lowest Common Ancestor](./lowest_common_ancestor.rs)
![alt text][common]

In graph theory and computer science, the lowest common ancestor (LCA) (also called least common ancestor) of two nodes v and w in a tree or directed acyclic graph (DAG) T is the lowest (i.e. deepest) node that has both v and w as descendants, where we define each node to be a descendant of itself (so if v has a direct connection from w, w is the lowest common ancestor).<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor)

### [Minimum Spanning Trees](./minimum_spanning_tree.rs)
![alt text][tree]

A minimum spanning tree (MST) or minimum weight spanning tree is a subset of the edges of a connected, edge-weighted undirected graph that connects all the vertices together, without any cycles and with the minimum possible total edge weight. That is, it is a spanning tree whose sum of edge weights is as small as possible. More generally, any edge-weighted undirected graph (not necessarily connected) has a minimum spanning forest, which is a union of the minimum spanning trees for its connected components.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Minimum_spanning_tree)

### [Prim's Algorithm](./prim.rs)
![alt text][prim]

In computer science, Prim's algorithm is a greedy algorithm that finds a minimum spanning tree for a weighted undirected graph. This means it finds a subset of the edges that forms a tree that includes every vertex, where the total weight of all the edges in the tree is minimized. The algorithm operates by building this tree one vertex at a time, from an arbitrary starting vertex, at each step adding the cheapest possible connection from the tree to another vertex.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Prim%27s_algorithm)

### [Prufer Code](./prufer_code.rs)
![alt text][prufer]

In combinatorial mathematics, the Prüfer sequence (also Prüfer code or Prüfer numbers) of a labeled tree is a unique sequence associated with the tree. The sequence for a tree on n vertices has length n − 2, and can be generated by a simple iterative algorithm. Prüfer sequences were first used by Heinz Prüfer to prove Cayley's formula in 1918.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Pr%C3%BCfer_sequence)

### [Tarjan's strongly connected components algorithm](./strongly_connected_components.rs)
![alt text][strong]

Tarjan's strongly connected components algorithm is an algorithm in graph theory for finding the strongly connected components (SCCs) of a directed graph. It runs in linear time, matching the time bound for alternative methods including Kosaraju's algorithm and the path-based strong component algorithm. The algorithm is named for its inventor, Robert Tarjan.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm)

### [Topological Sort](./topological_sort.rs)
![alt text][top]

In computer science, a topological sort or topological ordering of a directed graph is a linear ordering of its vertices such that for every directed edge (u,v) from vertex u to vertex v, u comes before v in the ordering. For instance, the vertices of the graph may represent tasks to be performed, and the edges may represent constraints that one task must be performed before another; in this application, a topological ordering is just a valid sequence for the tasks. Precisely, a topological sort is a graph traversal in which each node v is visited only after all its dependencies are visited. A topological ordering is possible if and only if the graph has no directed cycles, that is, if it is a directed acyclic graph (DAG). Any DAG has at least one topological ordering, and algorithms are known for constructing a topological ordering of any DAG in linear time. Topological sorting has many applications, especially in ranking problems such as feedback arc set. Topological sorting is possible even when the DAG has disconnected components.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Topological_sorting)



[ford]: https://upload.wikimedia.org/wikipedia/commons/thumb/7/77/Bellman%E2%80%93Ford_algorithm_example.gif/330px-Bellman%E2%80%93Ford_algorithm_example.gif
[search]: https://upload.wikimedia.org/wikipedia/commons/thumb/3/33/Breadth-first-tree.svg/450px-Breadth-first-tree.svg.png
[centroid]: https://media.geeksforgeeks.org/wp-content/cdn-uploads/centroidDecomposition1.png
[1search]: https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/Depth-first-tree.svg/375px-Depth-first-tree.svg.png
[Dijkstra]: https://upload.wikimedia.org/wikipedia/commons/5/57/Dijkstra_Animation.gif
[maxflow]: https://upload.wikimedia.org/wikipedia/commons/thumb/3/37/Dinic_algorithm_G1.svg/450px-Dinic_algorithm_G1.svg.png
[set]: https://he-s3.s3.amazonaws.com/media/uploads/a1f5858.jpg
[tic]: https://mathworld.wolfram.com/images/eps-svg/Tic-Tac-Toe_600.svg
[graph]: https://upload.wikimedia.org/wikipedia/commons/thumb/e/ee/Cayley%27s_formula_2-4.svg/330px-Cayley%27s_formula_2-4.svg.png
[light]: https://media.geeksforgeeks.org/wp-content/cdn-uploads/Heavy-Light-Decompostion.png
[common]: https://upload.wikimedia.org/wikipedia/commons/thumb/9/96/Lowest_common_ancestor.svg/210px-Lowest_common_ancestor.svg.png
[tree]: https://upload.wikimedia.org/wikipedia/commons/thumb/d/d2/Minimum_spanning_tree.svg/450px-Minimum_spanning_tree.svg.png
[prim]: https://upload.wikimedia.org/wikipedia/commons/thumb/9/9b/PrimAlgDemo.gif/300px-PrimAlgDemo.gif
[prufer]: https://upload.wikimedia.org/wikipedia/commons/thumb/2/24/Tree_graph.svg/243px-Tree_graph.svg.png
[strong]: https://upload.wikimedia.org/wikipedia/commons/thumb/6/60/Tarjan%27s_Algorithm_Animation.gif/375px-Tarjan%27s_Algorithm_Animation.gif
[top]: https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Directed_acyclic_graph_2.svg/465px-Directed_acyclic_graph_2.svg.png