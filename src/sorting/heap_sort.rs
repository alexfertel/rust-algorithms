use crate::sorting::traits::{InplaceSorter, Sorter};

pub struct HeapSort;

impl<T> InplaceSorter<T> for HeapSort
where
    T: Ord,
{
    fn sort_inplace(array: &mut [T]) {
        heap_sort(array);
    }
}

impl<T> Sorter<T> for HeapSort
where
    T: Ord + Clone,
{
    fn sort(array: &[T]) -> Vec<T> {
        let mut array = array.to_vec();
        heap_sort(&mut array);
        array
    }
}

fn heap_sort<T: Ord>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    heapify(array);

    let mut end = array.len() - 1;
    while end > 0 {
        array.swap(end, 0);
        end -= 1;
        siftdown(array, 0, end);
    }
}

fn heapify<T: Ord>(array: &mut [T]) {
    let start = (array.len() - 2) / 2;
    for i in (0..start + 1).rev() {
        siftdown(array, i, array.len() - 1);
    }
}

fn siftdown<T: Ord>(array: &mut [T], mut root: usize, end: usize) {
    while 2 * root < end {
        let child = 2 * root + 1;
        let mut swap = root;

        if array[swap] < array[child] {
            swap = child;
        }
        if child < end && array[swap] < array[child + 1] {
            swap = child + 1;
        }

        if swap == root {
            return;
        } else {
            array.swap(root, swap);
            root = swap;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::HeapSort;

    sorting_tests!(HeapSort::sort, heap_sort);
    sorting_tests!(HeapSort::sort_inplace, heap_sort_inplace, inplace);
}
