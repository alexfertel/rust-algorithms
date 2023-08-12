use crate::sorting::traits::Sorter;

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut j = i;
        // Move elements of arr[0..i-1],
        // that are greater than key, to one
        // position ahead of their
        // current position.
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Insertion sort divides the array into sorted and unsorted parts.
/// Values from the unsorted parts are placed in the correct position in the sorted part.
/// Time complexity is O(N^2)
/// Auxiliary space is O(1)
pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        insertion_sort(array);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::InsertionSort;

    sorting_tests!(InsertionSort::sort, insertion_sort);
    sorting_tests!(InsertionSort::sort_inplace, insertion_sort, inplace);
}
