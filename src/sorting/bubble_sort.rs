use crate::sorting::traits::{Sorter, InplaceSorter};

/// It sorts the array by repeatedly comparing the
/// adjacent elements and swapping them if they are
/// in the wrong order.
/// Time complexity is O(N^2)
/// Auxiliary space is O(1)
pub struct BubbleSort;

impl<T> InplaceSorter<T> for BubbleSort
    where
        T: Ord,
    {
    fn sort_inplace(array: &mut [T]) {
        for i in 0..array.len() {
            // Last i elements are already in place.
            for j in 0..array.len() - 1 - i {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                }
            }
        }
    }
}

impl<T> Sorter<T> for BubbleSort
    where
        T: Ord + Clone,
    {
    fn sort(array: &[T]) -> Vec<T> {
        for i in 0..array.len() {
            // Last i elements are already in place.
            for j in 0..array.len() - 1 - i {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                }
            }
        }
        array.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::BubbleSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(BubbleSort::sort, bubble_sort);
    sorting_tests!(BubbleSort::sort_inplace, bubble_sort_inplace, inplace);
}
