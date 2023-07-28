use crate::sorting::traits::{InplaceSorter, Sorter};

/// Insertion sort divides the array into sorted and unsorted parts.
/// Values from the unsorted parts are placed in the correct position in the sorted part.
/// Time complexity is O(N^2)
/// Auxiliary space is O(1)
pub struct InsertionSort;

impl<T> InplaceSorter<T> for InsertionSort
where
    T: Ord,
{
    fn sort_inplace(array: &mut [T]) {
        for i in 0..array.len() {
            let mut j = i;
            // Move elements of arr[0..i-1],
            // that are greater than key, to one
            // position ahead of their
            // current position.
            while j > 0 && array[j] < array[j - 1] {
                array.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

impl<T> Sorter<T> for InsertionSort
where
    T: Ord + Copy,
{
    fn sort(array: &[T]) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let mut arr = array.to_vec();
        InsertionSort::sort_inplace(&mut arr);
        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::InsertionSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(InsertionSort::sort, insertion_sort);
    sorting_tests!(InsertionSort::sort_inplace, insertion_sort_inplace, inplace);
}
