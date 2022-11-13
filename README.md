# Classic Algorithms in Rust

This repo contains the implementation of various classic algorithms for
educational purposes in [Rust](https://www.rust-lang.org/). It includes 
a comprehensive list of algorithms. Contributions are welcome!

The main goal right now is to improve docs, code readability and tests.

## Setup

This repo is only for educational purposes. It is meant to be used as a
reference material. Thus, it is written as a library instead of a binary.

The way to check the execution of an algorithm is running the tests,
which you can do using:

```bash
cargo test
```

## Algorithms

### Sorting Algorithms

- [x] [Bubble](./src/sorting/bubble_sort.rs)
- [X] [Bucket](./src/sorting/bucket_sort.rs)
- [x] [Cocktail-Shaker](./src/sorting/cocktail_shaker_sort.rs)
- [x] [Counting](./src/sorting/counting_sort.rs)
- [x] [Cycle](./src/sorting/cycle_sort.rs)
- [x] [Exchange](./src/sorting/exchange_sort.rs)
- [x] [Heap](./src/sorting/heap_sort.rs)
- [x] [Insertion](./src/sorting/insertion_sort.rs)
- [x] [Gnome](./src/sorting/gnome_sort.rs)
- [x] [Merge](./src/sorting/merge_sort.rs)
- [x] [Odd-even](./src/sorting/odd_even_sort.rs)
- [x] [Pancake](./src/sorting/pancake_sort.rs)
- [x] [Quick](./src/sorting/quick_sort.rs)
- [x] [Radix](./src/sorting/radix_sort.rs)
- [x] [Selection](./src/sorting/selection_sort.rs)
- [x] [Shell](./src/sorting/shell_sort.rs)
- [x] [Stooge](./src/sorting/stooge_sort.rs)
- [x] [Comb](./src/sorting/comb_sort.rs)
- [x] [Bucket](./src/sorting/bucket_sort.rs)
- [x] [Timsort](./src/sorting/tim_sort.rs)

### Graphs

- [x] [Dijkstra](./src/graph/dijkstra.rs)
- [x] [Kruskal's Minimum Spanning Tree](./src/graph/minimum_spanning_tree.rs)
- [x] [Prim's Minimum Spanning Tree](./src/graph/prim.rs)
- [x] [Breadth-First Search (BFS)](./src/graph/breadth_first_search.rs)
- [x] [Depth First Search (DFS)](./src/graph/depth_first_search.rs)
- [x] [Bellman-Ford](./src/graph/bellman_ford.rs)
- [x] [Prufer Code](./src/graph/prufer_code.rs)
- [x] [Lowest Common Ancestor](./src/graph/lowest_common_ancestor.rs)
- [x] [Heavy Light Decomposition](./src/graph/heavy_light_decomposition.rs)
- [x] [Tarjan's Strongly Connected Components](./src/graph/strongly_connected_components.rs)
- [x] [Topological sorting](./src/graph/topological_sort.rs)
- [x] [Centroid Decomposition](./src/graph/centroid_decomposition.rs)
- [x] [Dinic's Max Flow](./src/graph/dinic_maxflow.rs)

### Dynamic Programming

- [x] [0-1 Knapsack](./src/dynamic_programming/knapsack.rs)
- [x] [Edit Distance](./src/dynamic_programming/edit_distance.rs)
- [x] [Longest common subsequence](./src/dynamic_programming/longest_common_subsequence.rs)
- [x] [Longest continuous increasing subsequence](./src/dynamic_programming/longest_continuous_increasing_subsequence.rs)
- [x] [Longest increasing subsequence](./src/dynamic_programming/longest_increasing_subsequence.rs)
- [x] [K-Means Clustering](./src/general/kmeans.rs)
- [x] [Coin Change](./src/dynamic_programming/coin_change.rs)
- [x] [Rod Cutting](./src/dynamic_programming/rod_cutting.rs)
- [x] [Egg Dropping Puzzle](./src/dynamic_programming/egg_dropping.rs)
- [x] [Maximum Subarray](./src/dynamic_programming/maximum_subarray.rs)
- [x] [Is Subsequence](./src/dynamic_programming/is_subsequence.rs)
- [x] [Maximal Square](./src/dynamic_programming/maximal_square.rs)

### Data Structures

- [x] [Stack](./src/data_structures/stack.rs)
- [x] [Queue](./src/data_structures/queue.rs)
- [x] [Heap](./src/data_structures/heap.rs)
- [x] [Linked List](./src/data_structures/linked_list.rs)
- [x] [Graph](./src/data_structures/graph.rs)
  - [x] [Directed](./src/data_structures/graph.rs)
  - [x] [Undirected](./src/data_structures/graph.rs)
- [x] [Trie](./src/data_structures/trie.rs)
- [x] [Binary Search Tree](./src/data_structures/binary_search_tree.rs)
- [x] [B-Tree](./src/data_structures/b_tree.rs)
- [x] [AVL Tree](./src/data_structures/avl_tree.rs)
- [x] [RB Tree](./src/data_structures/rb_tree.rs)
- [X] [Stack using Linked List](./src/data_structures/stack_using_singly_linked_list.rs)
- [x] [Segment Tree](./src/data_structures/segment_tree.rs)
- [x] [Fenwick Tree](./src/data_structures/fenwick_tree.rs)
- [x] [Union-find](./src/data_structures/union_find.rs)

### Strings

- [ ] Naive
- [ ] Finite Automaton
- [x] [Aho-Corasick Algorithm](./src/string/aho_corasick.rs)
- [x] [Burrows-Wheeler transform](./src/string/burrows_wheeler_transform.rs)
- [x] [Knuth Morris Pratt](./src/string/knuth_morris_pratt.rs)
- [x] [Manacher](./src/string/manacher.rs)
- [x] [Rabin Carp](./src/string/rabin_karp.rs)
- [x] [Reverse](./src/string/reverse.rs)
- [x] [Hamming Distance](./src/string/hamming_distance.rs)

### General

- [x] [Convex Hull: Graham Scan](./src/general/convex_hull.rs)
- [x] [N-Queens Problem](./src/general/nqueens.rs)
- [ ] Graph Coloringp
- [x] [Tower of Hanoi](./src/general/hanoi.rs)
- [x] [Kmeans](./src/general/kmeans.rs)
- [x] [Two Sum](./src/general/two_sum.rs)
- [x] [Huffman Encoding](./src/general/huffman_encoding.rs)

### Ciphers

- [x] [Caesar](./src/ciphers/caesar.rs)
- [x] [Morse Code](./src/ciphers/morse_code.rs)
- [x] [Polybius](./src/ciphers/polybius.rs)
- [x] [SHA-2](./src/ciphers/sha256.rs)
- [x] [TEA](./src/ciphers/tea.rs)
- [x] [Transposition](./src/ciphers/transposition.rs)
- [x] [Vigenère](./src/ciphers/vigenere.rs)
- [x] [XOR](./src/ciphers/xor.rs)
- Rot13
  - [x] [Another Rot13](./src/ciphers/another_rot13.rs)
  - [x] [Rot13](./src/ciphers/rot13.rs)

### Bit Manipulation

- [x] [Bit Distance](./src/bit_manipulation/basic.rs)
- [x] [Bits Length](./src/bit_manipulation/basic.rs)
- [x] [Clear Bit](./src/bit_manipulation/basic.rs)
- [x] [Count Ones](./src/bit_manipulation/basic.rs)
- [x] [Divide By Two](./src/bit_manipulation/basic.rs)
- [x] [Get Bit](./src/bit_manipulation/basic.rs)
- [x] [Is Even](./src/bit_manipulation/basic.rs)
- [x] [Is Positive](./src/bit_manipulation/basic.rs)
- [x] [Is Power Of Two](./src/bit_manipulation/basic.rs)
- [x] [Multiply By Two](./src/bit_manipulation/basic.rs)
- [x] [Multiply Signed](./src/bit_manipulation/basic.rs)
- [x] [Multiply Unsigned](./src/bit_manipulation/basic.rs)
- [x] [Set Bit](./src/bit_manipulation/basic.rs)
- [x] [Twos Complement](./src/bit_manipulation/basic.rs)
- [x] [Update Bit](./src/bit_manipulation/basic.rs)

### Geometry

- [x] [Closest pair of 2D points](./src/geometry/closest_points.rs)


### Search

- [x] [Linear](./src/searching/linear_search.rs)
- [x] [Binary](./src/searching/binary_search.rs)
- [x] [Recursive Binary](./src/searching/binary_search_recursive.rs)
- [x] [Kth Smallest](./src/searching/kth_smallest.rs)
- [x] [Exponential](./src/searching/exponential_search.rs)
- [x] [Jump](./src/searching/jump_search.rs)
- [x] [Fibonacci](./src/searching/fibonacci_search.rs)
- [x] [Quick Select](./src/searching/quick_select.rs)

### Math
- [x] [Baby-Step Giant-Step Algorithm](./src/math/baby_step_giant_step.rs)
- [x] [Extended euclidean algorithm](./src/math/extended_euclidean_algorithm.rs)
- [x] [Gaussian Elimination](./src/math/gaussian_elimination.rs)
- [x] [Greatest common divisor](./src/math/greatest_common_divisor.rs)
- [x] [Greatest common divisor of n numbers](./src/math/gcd_of_n_numbers.rs)
- [x] [Least common multiple of n numbers](./src/math/lcm_of_n_numbers.rs)
- [x] [Miller Rabin primality test](./src/math/miller_rabin.rs)
- [x] [Pascal's triangle](./src/math/pascal_triangle.rs)
- [x] [Square root with Newton's method](./src/math/square_root.rs)
- [x] [Fast power algorithm](./src/math/fast_power.rs)
- [X] [Perfect number](./src/math/perfect_numbers.rs)
- [X] [Prime factors](./src/math/prime_factors.rs)
- [X] [Prime number](./src/math/prime_numbers.rs)
- [x] [Linear Sieve](./src/math/linear_sieve.rs)
- [x] [Pollard's Rho algorithm](./src/math/pollard_rho.rs)
- [x] [Quadratic Residue](./src/math/quadratic_residue.rs)
- [x] [Simpson's Rule for Integration](./src/math/simpson_integration.rs)
- [x] [Fast Fourier Transform](./src/math/fast_fourier_transform.rs)
- [x] [Armstrong Number](./src/math/armstrong_number.rs)
- [x] [Permuted Congruential Random Number Generator](./src/math/random.rs)
- [x] [Zeller's Congruence Algorithm](./src/math/zellers_congruence_algorithm.rs)
- [x] [Karatsuba Multiplication Algorithm](./src/math/karatsuba_multiplication.rs)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md)
