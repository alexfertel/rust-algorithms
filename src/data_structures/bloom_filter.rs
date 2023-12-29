//! Bloom Filter is a probabilistic data structure designed to determine whether a given element
//! is a member of a set. The main characteristic of the Bloom Filter is it may give false
//! positives but never false negatives. In other words, a query returns either "possibly in set"
//! or "definitely not in set".
//!
//! This uses the [BitVec](https://crates.io/crates/bitvec) crate to store the bits.
//!
//! Consider looking into [Fnv](https://crates.io/crates/fnv) crate for more efficient hashing.

use bitvec::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simple Bloom Filter implementation with a given size and number of hash functions.
/// Multiple hash functions are used to reduce the probability of false positives.
///
/// Example usage:
/// ```
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
/// use rust_algorithms::data_structures::BloomFilter;
///
/// fn main() {
///     // Define hash functions
///     let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
///         Box::new(|data| {
///             let mut hasher = DefaultHasher::new();
///             data.hash(&mut hasher);
///             hasher.finish()
///         }),
///         Box::new(|data| {
///             let mut hasher = DefaultHasher::new();
///             data.hash(&mut hasher);
///             hasher.finish() ^ 0xFFFFFFFFFFFFFFFF // XOR with a constant for diversity
///         }),
///     ];
///
///     // Create a new BloomFilter with a size of 100 bits and the hash functions
///     let mut bloom_filter = BloomFilter::new(100, hash_functions);
///
///     // Insert elements into the BloomFilter
///     bloom_filter.insert(&"apple");
///     bloom_filter.insert(&"banana");
///     bloom_filter.insert(&"cherry");
///
///     // Check if elements are contained in the BloomFilter
///     println!("Contains 'apple': {}", bloom_filter.contains(&"apple"));     // Should print true
///     println!("Contains 'orange': {}", bloom_filter.contains(&"orange"));   // Should print false
///     println!("Contains 'cherry': {}", bloom_filter.contains(&"cherry"));   // Should print true
/// }
/// ```

pub struct BloomFilter {
    /// Stores bits to indicate whether an element may be in the set
    bit_array: BitVec,
    /// Hash functions to use
    hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>>,
}

impl BloomFilter {
    /// Creates a new Bloom Filter with the given size and hash functions
    pub fn new(size: usize, hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>>) -> Self {
        BloomFilter {
            bit_array: bitvec![0; size],
            hash_functions,
        }
    }

    /// Inserts an element into the Bloom Filter
    /// Hashes the element using each hash function and sets the corresponding bit to true
    ///
    /// Time Complexity: O(k) where k is the number of hash functions
    pub fn insert<T>(&mut self, item: &T)
    where
        T: AsRef<[u8]> + Hash,
    {
        for hash_function in &self.hash_functions {
            let hash = Self::hash(item, hash_function);
            let index = hash % self.bit_array.len() as u64;
            self.bit_array.set(index as usize, true);
        }
    }

    /// Checks if an element may be in the Bloom Filter
    /// NOTE: `true` implies the element may be in the set, `false` implies the element is not in the set.
    /// The output is *not* deterministic.
    ///
    /// Time Complexity: O(k) where k is the number of hash functions
    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: AsRef<[u8]> + Hash,
    {
        for hash_function in &self.hash_functions {
            let hash = Self::hash(item, hash_function);
            let index = hash % self.bit_array.len() as u64;
            if !self.bit_array[index as usize] {
                return false;
            }
        }
        true
    }

    /// Hashes an element using the given hash function
    fn hash<T>(item: &T, hash_function: &Box<dyn Fn(&[u8]) -> u64>) -> u64
    where
        T: AsRef<[u8]> + Hash,
    {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        hash_function(&hash.to_be_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_insert_and_contains() {
        let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish()
            }),
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish() ^ 0xFFFFFFFFFFFFFFFF
            }),
        ];

        let mut bloom_filter = BloomFilter::new(100, hash_functions);

        assert!(!bloom_filter.contains(&"apple"));
        assert!(!bloom_filter.contains(&"banana"));
        assert!(!bloom_filter.contains(&"cherry"));

        bloom_filter.insert(&"apple");
        bloom_filter.insert(&"banana");
        bloom_filter.insert(&"cherry");

        assert!(bloom_filter.contains(&"apple"));
        assert!(bloom_filter.contains(&"banana"));
        assert!(bloom_filter.contains(&"cherry"));

        // Check that false positives are within an acceptable range
        assert!(!bloom_filter.contains(&"orange"));
        assert!(!bloom_filter.contains(&"grape"));
        assert!(!bloom_filter.contains(&"kiwi"));
    }

    #[test]
    fn test_false_positive_probability() {
        // Test the false positive probability by inserting a known set of elements
        // and checking for false positives with additional elements

        let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish()
            }),
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish() ^ 0xFFFFFFFFFFFFFFFF
            }),
        ];

        let mut bloom_filter = BloomFilter::new(100, hash_functions);

        // Insert known elements
        let known_elements = vec!["apple", "banana", "cherry"];
        for element in &known_elements {
            bloom_filter.insert(element);
        }

        // Test false positives with additional elements
        let false_positive_elements = vec!["orange", "grape", "kiwi"];
        for element in &false_positive_elements {
            assert!(
                !bloom_filter.contains(element),
                "False positive for: {}",
                element
            );
        }
    }

    #[test]
    fn test_hash_function_diversity() {
        // Test that hash functions produce diverse results for different elements

        let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish()
            }),
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish() ^ 0xFFFFFFFFFFFFFFFF
            }),
        ];

        let bloom_filter = BloomFilter::new(100, hash_functions);

        let element1 = "apple";
        let element2 = "banana";

        let hash1 = BloomFilter::hash(&element1, &bloom_filter.hash_functions[0]);
        let hash2 = BloomFilter::hash(&element2, &bloom_filter.hash_functions[0]);

        assert_ne!(
            hash1, hash2,
            "Hash function 1 produces the same hash for different elements"
        );

        let hash1 = BloomFilter::hash(&element1, &bloom_filter.hash_functions[1]);
        let hash2 = BloomFilter::hash(&element2, &bloom_filter.hash_functions[1]);

        assert_ne!(
            hash1, hash2,
            "Hash function 2 produces the same hash for different elements"
        );
    }

    #[test]
    fn test_hash_function_consistency() {
        // Test that hash functions produce consistent results for the same element

        let hash_functions: Vec<Box<dyn Fn(&[u8]) -> u64>> = vec![
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish()
            }),
            Box::new(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish() ^ 0xFFFFFFFFFFFFFFFF
            }),
        ];

        let bloom_filter = BloomFilter::new(100, hash_functions);

        let element = "apple";

        let hash1 = BloomFilter::hash(&element, &bloom_filter.hash_functions[0]);
        let hash2 = BloomFilter::hash(&element, &bloom_filter.hash_functions[0]);

        assert_eq!(
            hash1, hash2,
            "Hash function 1 produces different hashes for the same element"
        );

        let hash1 = BloomFilter::hash(&element, &bloom_filter.hash_functions[1]);
        let hash2 = BloomFilter::hash(&element, &bloom_filter.hash_functions[1]);

        assert_eq!(
            hash1, hash2,
            "Hash function 2 produces different hashes for the same element"
        );
    }

    /// more extensive test and contains test
    #[test]
    fn test_bloom_filter_extended() {
        /// Get a vector of hash functions (since they are closures, we can't clone them)
        fn get_hash_functions() -> Vec<Box<dyn Fn(&[u8]) -> u64>> {
            vec![
                Box::new(|data| {
                    let mut hasher = DefaultHasher::new();
                    data.hash(&mut hasher);
                    hasher.finish()
                }),
                Box::new(|data| {
                    let mut hasher = DefaultHasher::new();
                    data.hash(&mut hasher);
                    hasher.finish() ^ 0xFFFFFFFFFFFFFFFF
                }),
            ]
        }

        let mut bloom_filter = BloomFilter::new(100, get_hash_functions());

        // Ensure the filter is initially empty
        assert!(!bloom_filter.contains(&"apple"));
        assert!(!bloom_filter.contains(&"banana"));
        assert!(!bloom_filter.contains(&"cherry"));

        // Insert items into the Bloom filter
        bloom_filter.insert(&"apple");
        bloom_filter.insert(&"banana");
        bloom_filter.insert(&"cherry");

        // Check for false positives (items that were not inserted)
        assert!(!bloom_filter.contains(&"orange"));
        assert!(!bloom_filter.contains(&"grape"));
        assert!(!bloom_filter.contains(&"kiwi"));

        // Check for false negatives (items that were inserted)
        assert!(bloom_filter.contains(&"apple"));
        assert!(bloom_filter.contains(&"banana"));
        assert!(bloom_filter.contains(&"cherry"));

        // Create a new Bloom filter with a larger capacity
        let mut bloom_filter_large = BloomFilter::new(100, get_hash_functions());

        // Insert items into the larger Bloom filter
        bloom_filter_large.insert(&"orange");
        bloom_filter_large.insert(&"grape");
        bloom_filter_large.insert(&"kiwi");

        // Check for false positives in the larger filter
        assert!(bloom_filter_large.contains(&"orange"));
        assert!(bloom_filter_large.contains(&"grape"));
        assert!(bloom_filter_large.contains(&"kiwi"));

        // Check for false negatives in the larger filter
        assert!(!bloom_filter_large.contains(&"apple"));
        assert!(!bloom_filter_large.contains(&"banana"));
        assert!(!bloom_filter_large.contains(&"cherry"));

        // Check the accuracy of the original Bloom filter with new items
        assert!(!bloom_filter.contains(&"orange"));
        assert!(!bloom_filter.contains(&"grape"));
        assert!(!bloom_filter.contains(&"kiwi"));
    }
}
