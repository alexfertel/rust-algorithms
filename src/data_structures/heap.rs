/// Heap implementation.
/// 
/// This is an internal structure  used by the Min/Max Heap implementations.
struct Heap<T: Ord + Copy> {
    pq: Vec<T>,
    n: usize,
}

/// MaxHeap implementation.
/// 
/// # Examples:
/// 
/// ```rust
/// use rust_algorithms::data_structures::MaxHeap;
/// 
/// let mut heap = MaxHeap::<i32>::new();
/// heap.insert(1);
/// heap.insert(2);
/// heap.insert(3);
/// heap.insert(4);
/// heap.insert(5);
/// 
/// assert_eq!(heap.is_empty(), false);
/// assert_eq!(heap.size(), 5);
/// assert_eq!(heap.del_max(), 5);
/// assert_eq!(heap.del_max(), 4);
/// assert_eq!(heap.del_max(), 3);
/// assert_eq!(heap.del_max(), 2);
/// assert_eq!(heap.del_max(), 1);
/// assert_eq!(heap.is_empty(), true);
/// ```
pub struct MaxHeap<T: Ord + Copy> {
    heap: Heap<T>,
}

/// MinHeap implementation.
/// 
/// # Examples:
/// 
/// ```rust
/// use rust_algorithms::data_structures::MinHeap;
/// 
/// let mut heap = MinHeap::<i32>::new();
/// heap.insert(1);
/// heap.insert(2);
/// heap.insert(3);
/// heap.insert(4);
/// heap.insert(5);
/// 
/// assert_eq!(heap.is_empty(), false);
/// assert_eq!(heap.size(), 5);
/// assert_eq!(heap.del_min(), 1);
/// assert_eq!(heap.del_min(), 2);
/// assert_eq!(heap.del_min(), 3);
/// assert_eq!(heap.del_min(), 4);
/// assert_eq!(heap.del_min(), 5);
/// assert_eq!(heap.is_empty(), true);
/// ```
pub struct MinHeap<T: Ord + Copy> {
    heap: Heap<T>,
}


impl<T: Ord + Copy> Heap<T> {
    fn new() -> Heap<T> {
        Heap {
            pq: Vec::new(),
            n: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn size(&self) -> usize {
        self.n
    }

    fn insert(&mut self, key: T, less: fn(T, T) -> bool) {
        if self.is_empty() {
            self.pq.insert(0, key);
        }
        self.n += 1;
        self.pq.insert(self.n, key);
        self.swim(self.n, less);
    }

    fn del(&mut self, less: fn(T, T) -> bool) -> T {
        let item = self.peek();
        self.exch(1, self.pq.len() - 1);
        self.pq.remove(self.pq.len() - 1);
        self.n -= 1;
        self.sink(1, less);
        item
    }

    fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Heap is empty")
        }
        self.pq[1]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
    }

    fn swim(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while k > 1 && less(self.pq[k / 2], self.pq[k]) {
            self.pq.swap((k / 2) as usize, k as usize);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && less(self.pq[j], self.pq[j + 1]) {
                j += 1;
            }
            if !less(self.pq[k], self.pq[j]) {
                break;
            }
            self.pq.swap(k as usize, j as usize);
            k = j
        }
    }

    fn iter(&mut self) -> Vec<T> {
        self.pq[1..].to_vec()
    }
}

/// MaxHeap implementation.
impl<T: Ord + Copy> MaxHeap<T> {

    /// Creates a new `MaxHeap`` instance.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let heap = MaxHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.is_empty(), true);
    /// assert_eq!(heap.size(), 0);
    /// ```
    pub fn new() -> MaxHeap<T> {
        MaxHeap { heap: Heap::new() }
    }

    /// Inserts a new key into the `MaxHeap`.
    /// 
    /// # Arguments:
    /// 
    /// * `key` - The key to be inserted into the `MaxHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.is_empty(), false);
    /// assert_eq!(heap.size(), 5);
    /// ```
    pub fn insert(&mut self, key: T) {
        self.heap.insert(key, less_max);
    }

    /// Checks if the `MaxHeap` is empty.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.is_empty(), true);
    /// 
    /// heap.insert(1);
    /// 
    /// assert_eq!(heap.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the size of the `MaxHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.size(), 0);
    /// 
    /// heap.insert(1);
    /// 
    /// assert_eq!(heap.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.heap.size()
    }

    /// Gets the maximum key in the `MaxHeap`.
    /// 
    /// # Returns:
    /// 
    /// The maximum key in the `MaxHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.peek(), 5);
    /// ```
    pub fn peek(&self) -> T {
        self.heap.peek()
    }

    /// Deletes the maximum key in the `MaxHeap`.
    /// 
    /// # Returns:
    /// 
    /// The maximum key in the `MaxHeap`.
    /// 
    /// # Panics:
    /// 
    /// If the heap is empty.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.del_max(), 5);
    /// assert_eq!(heap.del_max(), 4);
    /// assert_eq!(heap.del_max(), 3);
    /// assert_eq!(heap.del_max(), 2);
    /// assert_eq!(heap.del_max(), 1);
    /// ```
    pub fn del_max(&mut self) -> T {
        self.heap.del(less_max)
    }

    /// Returns an iterator over the MaxHeap.
    /// 
    /// # Returns:
    /// 
    /// An iterator over the heap. The iterator will yield the keys in an arbitrary order.
    /// 
    /// # Examples:
    /// 
/// ```rust
    /// use rust_algorithms::data_structures::MaxHeap;
    /// 
    /// let mut heap = MaxHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// let mut heap_iter = heap.iter();
    /// 
    /// heap_iter.sort();
    /// let mut counter = 1;
    /// 
    /// for i in heap_iter.iter() {
    ///    assert_eq!(*i, counter);
    ///   counter += 1;
    /// }
    pub fn iter(&mut self) -> Vec<T> {
        self.heap.iter()
    }
}

/// MinHeap implementation.
impl<T: Ord + Copy> MinHeap<T> {

    /// Creates a new `MinHeap`` instance.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let heap = MinHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.is_empty(), true);
    /// assert_eq!(heap.size(), 0);
    /// ```
    pub fn new() -> MinHeap<T> {
        MinHeap { heap: Heap::new() }
    }

    /// Inserts a new key into the `MinHeap`.
    /// 
    /// # Arguments:
    /// 
    /// * `key` - The key to be inserted into the `MinHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.is_empty(), false);
    /// assert_eq!(heap.size(), 5);
    /// ```
    pub fn insert(&mut self, key: T) {
        self.heap.insert(key, less_min);
    }

    /// Checks if the `MinHeap` is empty.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.is_empty(), true);
    /// 
    /// heap.insert(1);
    /// 
    /// assert_eq!(heap.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the size of the `MinHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// 
    /// assert_eq!(heap.size(), 0);
    /// 
    /// heap.insert(1);
    /// 
    /// assert_eq!(heap.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.heap.size()
    }

    /// Gets the minimum key in the `MinHeap`.
    /// 
    /// # Returns:
    /// 
    /// The minimum key in the `MinHeap`.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.peek(), 1);
    /// 
    /// ```
    pub fn peek(&self) -> T {
        self.heap.peek()
    }

    /// Deletes the minimum key in the `MinHeap`.
    /// 
    /// # Returns:
    /// 
    /// The minimum key in the `MinHeap`.
    /// 
    /// # Panics:
    /// 
    /// If the heap is empty.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// assert_eq!(heap.del_min(), 1);
    /// assert_eq!(heap.del_min(), 2);
    /// assert_eq!(heap.del_min(), 3);
    /// assert_eq!(heap.del_min(), 4);
    /// assert_eq!(heap.del_min(), 5);
    /// ```
    pub fn del_min(&mut self) -> T {
        self.heap.del(less_min)
    }

    /// Returns an iterator over the MinHeap.
    /// 
    /// # Returns:
    /// 
    /// An iterator over the heap. The iterator will yield the keys in an arbitrary order.
    /// 
    /// # Examples:
    /// 
    /// ```rust
    /// use rust_algorithms::data_structures::MinHeap;
    /// 
    /// let mut heap = MinHeap::<i32>::new();
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(3);
    /// heap.insert(4);
    /// heap.insert(5);
    /// 
    /// let mut heap_iter = heap.iter();
    /// 
    /// heap_iter.sort();
    /// let mut counter = 1;
    /// 
    /// for i in heap_iter.iter() {
    ///    assert_eq!(*i, counter);
    ///   counter += 1;
    /// }
    pub fn iter(&mut self) -> Vec<T> {
        self.heap.iter()
    }
}

fn less_max<T: Ord + Copy>(i: T, j: T) -> bool {
    i.lt(&j)
}

fn less_min<T: Ord + Copy>(i: T, j: T) -> bool {
    !i.lt(&j)
}
