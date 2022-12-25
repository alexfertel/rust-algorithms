### [Bogo Sort](./bogo_sort.rs)

From [Wikipedia][bogosort-wiki]: In computer science, bogosort (also known as permutation sort, stupid sort, slowsort or bozosort) is a sorting algorithm based on the generate and test paradigm. The function successively generates permutations of its input until it finds one that is sorted. It is not considered useful for sorting, but may be used for educational purposes, to contrast it with more efficient algorithms.

Two versions of this algorithm exist: a deterministic version that enumerates all permutations until it hits a sorted one,and a randomized version that randomly permutes its input. An analogy for the working of the latter version is to sort a deck of cards by throwing the deck into the air, picking the cards up at random, and repeating the process until the deck is sorted. Its name is a portmanteau of the words bogus and sort.

[bogosort-wiki]:https://en.wikipedia.org/wiki/Bogosort

### [Bucket_Sort](./bucket_sort.rs)

From [Wikipedia][bucketsort-wiki]: Bucket sort, or bin sort, is a sorting algorithm that works by distributing the elements of an array into a number of buckets. Each bucket is then sorted individually, either using a different sorting algorithm, or by recursively applying the bucket sorting algorithm. It is a distribution sort, a generalization of pigeonhole sort that allows multiple keys per bucket, and is a cousin of radix sort in the most-to-least significant digit flavor. Bucket sort can be implemented with comparisons and therefore can also be considered a comparison sort algorithm. The computational complexity depends on the algorithm used to sort each bucket, the number of buckets to use, and whether the input is uniformly distributed.

Bucket sort works as follows:

Set up an array of initially empty "buckets".
Scatter: Go over the original array, putting each object in its bucket.
Sort each non-empty bucket.
Gather: Visit the buckets in order and put all elements back into the original array.

Optimization:
A common optimization is to put the unsorted elements of the buckets back in the original array first, then run insertion sort over the complete array; because insertion sort's runtime is based on how far each element is from its final position, the number of comparisons remains relatively small, and the memory hierarchy is better exploited by storing the list contiguously in memory.

If the input distribution is known or can be estimated, buckets can often be chosen which contain constant density (rather than merely having constant size). This allows O(n) average time complexity even without uniformly distributed input.

[bucketsort-wiki]: https://en.wikipedia.org/wiki/Bucket_sort

### [Bubble Sort](./bubble_sort.rs)

Bubble Sort is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in the wrong order.

1. First Iteration (Compare and Swap)

* Starting from the first index, compare the first and the second elements.
* If the first element is greater than the second element, they are swapped.
* Now, compare the second and the third elements. Swap them if they are not in order.
* The above process goes on until the last element.

2. Remaining Iteration

* The same process goes on for the remaining iterations.
* After each iteration, the largest element among the unsorted elements is placed at the end.
* In each iteration, the comparison takes place up to the last unsorted element.
* The array is sorted when all the unsorted elements are placed at their correct positions.

![alt text](https://technologystrive.com/wp-content/uploads/2021/09/Bubble-Sort-Example-Iteration-1-1536x1070.png?v=1632804991)

__Properties__
* Worst/Average time complexity : O(N2).
* This algorithm is not suitable for large data sets.

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Bubble_sort)
* [Geeksforgeeks](https://www.geeksforgeeks.org/bubble-sort/)
* [Programiz](https://www.programiz.com/dsa/bubble-sort)

### [Cocktail_Shaker_Sort](./cocktail_shaker_sort.rs)

Cocktail Shaker Sort, also called bidirectional bubble sort, iterates through a list from top to bottom and then from bottom to top. It is highly performant when the items in the list are partially sorted. The complexity of any list is O(n^2), but it approaches O(n) if the distance of each item from its final position in the ordered list is relatively small.

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)
* [Geeksforgeeks](https://www.geeksforgeeks.org/quick-sort/)
* [Baeldung](https://www.baeldung.com/cs/cocktail-sort)

### [Comb_Sort](./comb_sort.rs)

Comb Sort is another optimization of Bubble Sort, and achieves this by eliminating small values towards the end of the list. It also uses a shrink-factor (k), which is used in the inner loop of Bubble Sort that performs the swapping of elements. The gap between elements that are compared in Bubble Sort is always 1, but the Comb Sort allows for the gap to be larger than 1 and then shrink by (k) for each iteration. (k) highly affects the efficiency of the algorithm, and is ideal at 1.3.

__Properties__
* Best time Complexity : O(n)
* Worst time Complexity: O(n^2)

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Comb_sort)
* [Geeksforgeeks](https://www.geeksforgeeks.org/comb-sort/)
* [javatpoint](https://www.javatpoint.com/comb-sort)


### [Quick Sort](./quick_sort.rs)

QuickSort is a Divide and Conquer algorithm. It picks an element as a pivot and partitions the given array around the picked pivot. There are 4 common ways of selecting a pivot:

* Pick the first element
* Pick the last element
* Pick a random element
* Pick the median element

The key process in quickSort is a partition(). The target of partitions is, given an array and an element x of an array as the pivot, put x at its correct position in a sorted array and put all smaller elements (smaller than x) before x, and put all greater elements (greater than x) after x. All this should be done in linear time.

![alt text](https://favtutor.com/resources/images/uploads/Quick_sort_algorithm.png)

__Properties__
* Average time complexity : O(n log n)
* Worst time complexity: O(n^2)

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Quicksort)
* [Geeksforgeeks](https://www.geeksforgeeks.org/quick-sort/)
* [Mygreatlearning](https://www.mygreatlearning.com/blog/quick-sort-algorithm/)


### [Selection Sort](./selection_sort.rs)

The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from the unsorted part and putting it at the beginning. 

The algorithm maintains two subarrays in a given array.

* The subarray which already sorted. 
* The remaining subarray was unsorted.

In every iteration of the selection sort, the minimum element (considering ascending order) from the unsorted subarray is picked and moved to the sorted subarray. 

![alt text](https://camo.githubusercontent.com/42802128a959e4df44e88a889a4f04ee19073cc038f92b4dfbe8cb0342a649d9/687474703a2f2f692e696d6775722e636f6d2f726e5266756d772e6a7067)

__Properties__
* Worst/Average time complexity : Ο(n2),  where n is the number of items

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Selection_sort)
* [Geeksforgeeks](https://www.geeksforgeeks.org/selection-sort/)
* [Programiz](https://www.programiz.com/dsa/selection-sort)



