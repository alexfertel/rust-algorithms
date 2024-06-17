## General

### [Convex Hull](./convex_hull.rs)
![alt text][convexhull]

In geometry, the convex hull, convex envelope or convex closure of a shape is the smallest convex set that contains it. The convex hull may be defined either as the intersection of all convex sets containing a given subset of a Euclidean space, or equivalently as the set of all convex combinations of points in the subset. For a bounded subset of the plane, the convex hull may be visualized as the shape enclosed by a rubber band stretched around the subset.<br>

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Convex_hull_algorithms)
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Convex_hull)

### [Graph Coloring](./graph_coloring.rs)
In graph theory, graph coloring is a special case of graph labeling; it is an assignment of labels traditionally called "colors" to elements of a graph subject to certain constraints. In its simplest form, it is a way of coloring the vertices of a graph such that no two adjacent vertices are of the same color; this is called a vertex coloring. Similarly, an edge coloring assigns a color to each edge so that no two adjacent edges are of the same color, and a face coloring of a planar graph assigns a color to each face or region so that no two faces that share a boundary have the same color.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Graph_coloring)

### [Tower of Hanoi](./hanoi.rs)
![alt text][Hanoi]

The Tower of Hanoi (also called The problem of Benares Temple[1] or Tower of Brahma or Lucas' Tower[2] and sometimes pluralized as Towers, or simply pyramid puzzle[3]) is a mathematical game or puzzle consisting of three rods and a number of disks of various diameters, which can slide onto any rod. The puzzle begins with the disks stacked on one rod in order of decreasing size, the smallest at the top, thus approximating a conical shape. The objective of the puzzle is to move the entire stack to one of the other rods, obeying the following rules:[4]

Only one disk may be moved at a time.
Each move consists of taking the upper disk from one of the stacks and placing it on top of another stack or on an empty rod.
No disk may be placed on top of a disk that is smaller than it.
With three disks, the puzzle can be solved in seven moves. The minimal number of moves required to solve a Tower of Hanoi puzzle is 2n − 1, where n is the number of disks.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Tower_of_Hanoi)

### [Huffman coding](./huffman_encoding.rs)
![alt text][Huffman]

In computer science and information theory, a Huffman code is a particular type of optimal prefix code that is commonly used for lossless data compression. The process of finding or using such a code is Huffman coding, an algorithm developed by David A. Huffman while he was a Sc.D. student at MIT, and published in the 1952 paper "A Method for the Construction of Minimum-Redundancy Codes".

The output from Huffman's algorithm can be viewed as a variable-length code table for encoding a source symbol (such as a character in a file). The algorithm derives this table from the estimated probability or frequency of occurrence (weight) for each possible value of the source symbol. As in other entropy encoding methods, more common symbols are generally represented using fewer bits than less common symbols. Huffman's method can be efficiently implemented, finding a code in time linear to the number of input weights if these weights are sorted.[2] However, although optimal among methods encoding symbols separately, Huffman coding is not always optimal among all compression methods - it is replaced with arithmetic coding[3] or asymmetric numeral systems if a better compression ratio is required.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Huffman_coding)

### [k-means clustering](./kmeans.rs)
![alt text][kmeans]

k-means clustering is a method of vector quantization, originally from signal processing, that aims to partition n observations into k clusters in which each observation belongs to the cluster with the nearest mean (cluster centers or cluster centroid), serving as a prototype of the cluster. This results in a partitioning of the data space into Voronoi cells. k-means clustering minimizes within-cluster variances (squared Euclidean distances), but not regular Euclidean distances, which would be the more difficult Weber problem: the mean optimizes squared errors, whereas only the geometric median minimizes Euclidean distances. For instance, better Euclidean solutions can be found using k-medians and k-medoids.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/K-means_clustering)

### [n queens](./nqueens.rs)
![alt text][nqueens]

The N Queen is the problem of placing N chess queens on an N×N chessboard so that no two queens attack each other.

###### Source: [Geeks for Geeks](https://www.geeksforgeeks.org/n-queen-problem-backtracking-3/)

### [2 Sum](./two_sum.rs)
![alt text][2sum]

Given an array A[] of n numbers and another number x, the task is to check whether or not there exist two elements in A[] whose sum is exactly x.

###### Source: [Geeks for Geeks](https://www.geeksforgeeks.org/check-if-pair-with-given-sum-exists-in-array/)

[convexhull]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/8e/Extreme_points.svg/330px-Extreme_points.svg.png
[Hanoi]: https://upload.wikimedia.org/wikipedia/commons/thumb/0/07/Tower_of_Hanoi.jpeg/450px-Tower_of_Hanoi.jpeg
[Huffman]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/82/Huffman_tree_2.svg/330px-Huffman_tree_2.svg.png
[kmeans]: https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/K-means_convergence.gif/330px-K-means_convergence.gif
[nqueens]: https://media.geeksforgeeks.org/wp-content/uploads/20230814111624/N-Queen-Problem.png
