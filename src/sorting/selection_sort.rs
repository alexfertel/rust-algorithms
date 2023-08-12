use crate::sorting::traits::Sorter;

fn selection_sort<T: Ord>(array: &mut [T]) {
    // Loop through each element in the array.
    for i in 0..array.len() {
        // The current element is the starting minimum element.
        let mut smallest_idx = i;
        // Loop through the remaining elements, if any of them is less than the current minimum, we update the current minimum.
        for j in i + 1..array.len() {
            if array[j] < array[smallest_idx] {
                smallest_idx = j;
            }
        }
        // We can then swap the minimum element with the current element.
        array.swap(i, smallest_idx);
    }
}

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        selection_sort(array);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::SelectionSort;

    sorting_tests!(SelectionSort::sort, selection_sort);
    sorting_tests!(SelectionSort::sort_inplace, selection_sort, inplace);
}
