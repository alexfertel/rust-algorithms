use crate::sorting::traits::{InplaceSorter, Sorter};

// The Merge Sort algorithm is a sorting algorithm that is based on the Divide and Conquer paradigm.
// The Time complexity is `O(nlog(n))` where n is the length of the array.
// Auxillary Space required is `O(n)` Since all the elements are copied to the auxillary space.
pub struct MergeSort;

impl<T> InplaceSorter<T> for MergeSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        let result = merge_sort(array);
        array.copy_from_slice(&result);
    }
}

impl<T> Sorter<T> for MergeSort
where
    T: Ord + Copy,
{
    fn sort(array: &[T]) -> Vec<T> {
        merge_sort(array)
    }
}

fn merge_sort<T: Ord + Copy>(array: &[T]) -> Vec<T> {
    if array.len() < 2 {
        return array.to_vec();
    }
    let middle = array.len() / 2;
    let left = merge_sort(&array[..middle]);
    let right = merge_sort(&array[middle..]);
    merge(&left, &right)
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    while let (Some(left_val), Some(right_val)) = (left_next, right_next) {
        if left_val <= right_val {
            result.push(*left_val);
            left_next = left_iter.next();
        } else {
            result.push(*right_val);
            right_next = right_iter.next();
        }
    }

    result.extend(left_next);
    result.extend(right_next);
    result
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::MergeSort;

    sorting_tests!(MergeSort::sort, merge_sort);
    sorting_tests!(MergeSort::sort_inplace, merge_sort_inplace, inplace);
}
