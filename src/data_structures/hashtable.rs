use std::collections::LinkedList;

/// The growth factor of the hash table when resizing.
const GROWTH_FACTOR: usize = 2;

/// The load factor bound of the hash table. The hash table will resize itself when the number of
/// elements exceeds the load factor bound.
const LOAD_FACTOR_BOUND: f64 = 0.75;

/// The initial capacity of the hash table.
const INITIAL_CAPACITY: usize = 3000;

/// A hash table implementation with separate chaining. It uses a linked list to store elements
/// with the same hash.
/// 
/// # Notes:
/// 
/// The hash table will resize itself when the number of elements exceeds the load factor bound.
/// The hash table will grow by a factor of 2 when resizing.
/// The hash table uses a default initial capacity of 3000.
/// 
/// # Examples:
/// 
/// ```rust
/// use rust_algorithms::data_structures::HashTable;
/// 
/// let mut hash_table = HashTable::new();
/// 
/// hash_table.insert(1usize, 10);
/// let result = hash_table.search(1);
/// 
/// assert_eq!(result, Some(&10));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

/// Implement Default for HashTable
impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {

    /// Create a new HashTable with the default initial capacity.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::HashTable;
    /// 
    /// let hash_table: HashTable<usize, usize> = HashTable::default();
    /// 
    /// assert!(hash_table.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for types that can be hashed.
pub trait Hashable {
    fn hash(&self) -> usize;
}

/// Implement Hashable for usize
/// This is useful for testing purposes but doesn't provide a meaningful hash function.
impl Hashable for usize {
    fn hash(&self) -> usize {
        *self
    }
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {

    /// Create a new HashTable with the default initial capacity.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::HashTable;
    /// 
    /// let hash_table = HashTable::<usize, usize>::new();
    /// 
    /// assert!(hash_table.is_empty());
    /// ```
    pub fn new() -> HashTable<K, V> {
        let initial_capacity = INITIAL_CAPACITY;
        let mut elements = Vec::with_capacity(initial_capacity);

        for _ in 0..initial_capacity {
            elements.push(LinkedList::new());
        }

        HashTable { elements, count: 0 }
    }

    /// Returns the number of elements in the hash table.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::HashTable;
    /// 
    /// let mut hash_table = HashTable::<usize, usize>::new();
    /// 
    /// assert_eq!(hash_table.is_empty(), true);
    /// 
    /// hash_table.insert(1usize, 10);
    /// 
    /// assert_eq!(hash_table.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Insert a key-value pair into the hash table.
    /// 
    /// # Arguments:
    /// 
    /// * `key` - The key to insert.
    /// * `value` - The value to insert.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::HashTable;
    /// 
    /// let mut hash_table = HashTable::new();
    /// 
    /// hash_table.insert(1usize, 10);
    /// let result = hash_table.search(1);
    /// 
    /// assert_eq!(result, Some(&10));
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        if self.count >= self.elements.len() * LOAD_FACTOR_BOUND as usize {
            self.resize();
        }
        let index = key.hash() % self.elements.len();
        self.elements[index].push_back((key, value));
        self.count += 1;
    }

    /// Search for a key in the hash table.
    /// 
    /// # Arguments:
    /// 
    /// * `key` - The key to search for.
    /// 
    /// # Returns:
    /// 
    /// An Option containing a reference to the value if the key is found, or None if the key is not
    /// found.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::HashTable;
    /// 
    /// let mut hash_table = HashTable::new();
    /// 
    /// hash_table.insert(1usize, 10);
    /// let result = hash_table.search(1);
    /// 
    /// assert_eq!(result, Some(&10));
    /// ```
    pub fn search(&self, key: K) -> Option<&V> {
        let index = key.hash() % self.elements.len();
        self.elements[index]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn resize(&mut self) {
        let new_size = self.elements.len() * GROWTH_FACTOR;
        let mut new_elements = Vec::with_capacity(new_size);

        for _ in 0..new_size {
            new_elements.push(LinkedList::new());
        }

        for old_list in self.elements.drain(..) {
            for (key, value) in old_list {
                let new_index = key.hash() % new_size;
                new_elements[new_index].push_back((key, value));
            }
        }

        self.elements = new_elements;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let mut hash_table = HashTable::new();
        let initial_capacity = hash_table.elements.capacity();

        for i in 0..initial_capacity * LOAD_FACTOR_BOUND as usize + 1 {
            hash_table.insert(i, i + 10);
        }

        assert!(hash_table.elements.capacity() > initial_capacity);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut hash_table = HashTable::new();
        let key = 1;
        let value = 10;

        hash_table.insert(key, value);
        let result = hash_table.search(2);

        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
        let mut hash_table = HashTable::new();
        for i in 0..10 {
            hash_table.insert(i, i + 100);
        }

        for i in 0..10 {
            let result = hash_table.search(i);
            assert_eq!(result, Some(&(i + 100)));
        }
    }

    #[test]
    fn test_not_overwrite_existing_key() {
        let mut hash_table = HashTable::new();
        hash_table.insert(1, 100);
        hash_table.insert(1, 200);

        let result = hash_table.search(1);
        assert_eq!(result, Some(&100));
    }

    #[test]
    fn test_empty_search() {
        let hash_table: HashTable<usize, usize> = HashTable::new();
        let result = hash_table.search(1);

        assert_eq!(result, None);
    }
}
