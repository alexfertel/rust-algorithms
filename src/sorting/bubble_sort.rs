use crate::sorting::traits::{Sorter, InplaceSorter};

/// It sorts the array by repeatedly comparing the
/// adjacent elements and swapping them if they are
/// in the wrong order.
/// Time complexity is O(N^2)
/// Auxiliary space is O(1)
pub struct BubbleSort;

impl<T> InplaceSorter<T> for BubbleSort {
    fn sort_inplace(array: &mut [T])
    where
        T: Ord,
    {
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

impl<T> Sorter<T> for BubbleSort {
    fn sort(array: &[T]) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let mut arr = array.to_vec();
        BubbleSort::sort_inplace(&mut arr);
        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::BubbleSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(BubbleSort::sort, bubble_sort);
    sorting_tests!(BubbleSort::sort_inplace, bubble_sort_inplace, inplace);
}
