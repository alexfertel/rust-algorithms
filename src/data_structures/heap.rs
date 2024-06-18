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

impl<T: Ord + Copy> MaxHeap<T> {
    pub fn new() -> MaxHeap<T> {
        MaxHeap { heap: Heap::new() }
    }

    pub fn insert(&mut self, key: T) {
        self.heap.insert(key, less_max);
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    pub fn peek(&self) -> T {
        self.heap.peek()
    }

    pub fn del_max(&mut self) -> T {
        self.heap.del(less_max)
    }

    pub fn iter(&mut self) -> Vec<T> {
        self.heap.iter()
    }
}

impl<T: Ord + Copy> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { heap: Heap::new() }
    }

    pub fn insert(&mut self, key: T) {
        self.heap.insert(key, less_min);
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    pub fn peek(&self) -> T {
        self.heap.peek()
    }

    pub fn del_min(&mut self) -> T {
        self.heap.del(less_min)
    }

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

#[cfg(test)]
mod tests {
    use crate::data_structures::heap;
    
    #[test]
    fn min_heap_iter() {
        let mut heap = heap::MinHeap::<i32>::new();
        heap.insert(1);
        heap.insert(2);
        heap.insert(3);
        heap.insert(4);
        heap.insert(5);

        let heap_iter = heap.iter();
        let mut counter = 1;
        for i in heap_iter {
            assert_eq!(i, counter);
            counter += 1;
        }
    }

}
