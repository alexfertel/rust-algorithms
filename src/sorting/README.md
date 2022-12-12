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



