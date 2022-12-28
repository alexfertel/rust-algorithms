// It sorts the array by repeatedly comparing the
// adjacent elements and swapping them if they are
// in the wrong order.
// Time complexity is O(N^2)
// Auxiliary space is O(1)
use super::traits::MutableSorter;

pub struct BubbleSort;

impl<T> MutableSorter<T> for BubbleSort {
    fn sort(array: &mut [T])
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

#[cfg(test)]
mod tests {
    use super::super::traits::MutableSorter;
    use super::BubbleSort;
    sorting_tests!(BubbleSort::sort, inplace);
}
