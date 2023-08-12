use crate::sorting::traits::Sorter;

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        // Last i elements are already in place.
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// It sorts the array by repeatedly comparing the
/// adjacent elements and swapping them if they are
/// in the wrong order.
/// Time complexity is O(N^2)
/// Auxiliary space is O(1)
pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort
where
    T: Ord + Copy
{
    fn sort_inplace(array: &mut [T]) {
        bubble_sort(array);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::BubbleSort;

    sorting_tests!(BubbleSort::sort, bubble_sort);
    sorting_tests!(BubbleSort::sort_inplace, bubble_sort, inplace);
}
