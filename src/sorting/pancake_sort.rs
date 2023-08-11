use crate::sorting::traits::{InplaceSorter, Sorter};
use std::cmp;

pub struct PancakeSort;

impl<T> InplaceSorter<T> for PancakeSort
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    fn sort_inplace(arr: &mut [T]) {
        pancake_sort(arr);
    }
}

impl<T> Sorter<T> for PancakeSort
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    fn sort(arr: &[T]) -> Vec<T> {
        pancake_sort(&mut arr.to_vec())
    }
}

fn pancake_sort<T>(arr: &mut [T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    let len = arr.len();
    if len < 2 {
        arr.to_vec();
    }
    for i in (0..len).rev() {
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();
        if max_index != i {
            arr[0..max_index + 1].reverse();
            arr[0..i + 1].reverse();
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::PancakeSort;

    sorting_tests!(PancakeSort::sort, pancake_sort);
    sorting_tests!(PancakeSort::sort_inplace, pancake_sort_inplace, inplace);
}
