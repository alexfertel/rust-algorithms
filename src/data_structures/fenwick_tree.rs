use std::ops::{Add, AddAssign};

/// A Fenwick Tree (also known as a Binary Indexed Tree) is a data structure
/// that can efficiently update elements and calculate prefix sums in a table of numbers.
///
/// If we have an array arr[0 . . . n-1]. We would like to:
/// 1. Compute the sum of the first i elements.
/// 2. Modify the value of a specified element of the array arr[i] = x
///    where 0 <= i <= n-1.
///
/// A simple solution is to run a loop from 0 to i-1 and calculate the sum of the elements.
/// To update a value, simply do arr[i] = x. The first operation takes O(n) time and the
/// second operation takes O(1) time. Another simple solution is to create an extra array and
/// store the sum of the first i elements at the i-th index in this new array. The sum of a given
/// range can be calculated in O(1) time, but the update operation takes O(n) time now. This works
/// well if the number of query operations is large and the number of update operations is small.
///
/// The Fenwick tree provides a way to represent an array of numbers and perform two operations
/// in O(log n) time.
///
/// The two operations are:
/// 1. Update: Given a number x and an index i, we need to add x to the i-th element.
/// 2. Query: Given an index i, we need to calculate the prefix sum of the first i elements.
///
/// The Fenwick tree is represented as an array of n elements. The tree structure allows us to
/// perform efficient queries and updates. The tree is constructed in such a way that the value
/// of each node is the sum of the values of the nodes in its subtree.
///
pub struct FenwickTree<T: Add + AddAssign + Copy + Default> {
    data: Vec<T>,
}

impl<T: Add<Output = T> + AddAssign + Copy + Default> FenwickTree<T> {
    /// Create a new FenwickTree with length `len`
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the FenwickTree
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::FenwickTree;
    ///
    /// let mut ft = FenwickTree::with_len(10);
    /// ft.add(0, 1);
    /// ft.add(1, 2);
    ///
    /// assert_eq!(ft.prefix_sum(0), 1);
    /// assert_eq!(ft.prefix_sum(1), 3);
    /// ```
    pub fn with_len(len: usize) -> Self {
        FenwickTree {
            data: vec![T::default(); len + 1],
        }
    }

    /// Add `val` to the `i`-th element
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the element to add `val` to
    /// * `val` - The value to add to the `i`-th element
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::FenwickTree;
    ///
    /// let mut ft = FenwickTree::with_len(10);
    /// ft.add(0, 3);
    /// ft.add(1, 2);
    ///
    /// assert_eq!(ft.prefix_sum(0), 3);
    /// assert_eq!(ft.prefix_sum(1), 5);
    /// ```
    pub fn add(&mut self, i: usize, val: T) {
        assert!(i < self.data.len());

        let mut i = i + 1;

        while i < self.data.len() {
            self.data[i] += val;
            i += lowbit(i);
        }
    }

    /// Get the prefix sum of the first `i` elements
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the last element to calculate the prefix sum of
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::FenwickTree;
    ///
    /// let mut ft = FenwickTree::with_len(10);
    /// ft.add(0, 1);
    /// ft.add(1, 2);
    /// ft.add(2, 3);
    /// ft.add(3, 4);
    /// ft.add(4, 5);
    /// ft.add(5, 6);
    /// ft.add(6, 7);
    /// ft.add(7, 8);
    /// ft.add(8, 9);
    /// ft.add(9, 10);
    ///
    /// assert_eq!(ft.prefix_sum(0), 1);
    /// assert_eq!(ft.prefix_sum(1), 3);
    /// assert_eq!(ft.prefix_sum(2), 6);
    /// assert_eq!(ft.prefix_sum(3), 10);
    /// assert_eq!(ft.prefix_sum(4), 15);
    /// assert_eq!(ft.prefix_sum(5), 21);
    /// assert_eq!(ft.prefix_sum(6), 28);
    /// assert_eq!(ft.prefix_sum(7), 36);
    /// assert_eq!(ft.prefix_sum(8), 45);
    /// assert_eq!(ft.prefix_sum(9), 55);
    /// ```
    pub fn prefix_sum(&self, i: usize) -> T {
        assert!(i < self.data.len());

        let mut i = i + 1;
        let mut res = T::default();

        while i > 0 {
            res += self.data[i];
            i -= lowbit(i);
        }

        res
    }
}

/// get the lowest bit of `i`
const fn lowbit(x: usize) -> usize {
    let x = x as isize;
    (x & (-x)) as usize
}
