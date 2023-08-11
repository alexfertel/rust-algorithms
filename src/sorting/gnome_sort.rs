use crate::sorting::traits::{InplaceSorter, Sorter};
use std::cmp;

pub struct GnomeSort;

impl<T> InplaceSorter<T> for GnomeSort
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    fn sort_inplace(arr: &mut [T]) {
        let mut i: usize = 1;
        let mut j: usize = 2;

        while i < arr.len() {
            if arr[i - 1] < arr[i] {
                i = j;
                j = i + 1;
            } else {
                arr.swap(i - 1, i);
                i -= 1;
                if i == 0 {
                    i = j;
                    j += 1;
                }
            }
        }
    }
}

impl<T> Sorter<T> for GnomeSort
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let mut arr = arr.to_vec();
        let mut i: usize = 1;
        let mut j: usize = 2;

        while i < arr.len() {
            if arr[i - 1] < arr[i] {
                i = j;
                j = i + 1;
            } else {
                arr.swap(i - 1, i);
                i -= 1;
                if i == 0 {
                    i = j;
                    j += 1;
                }
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::GnomeSort;

    sorting_tests!(GnomeSort::sort, gnome_sort);
    sorting_tests!(GnomeSort::sort_inplace, gnome_sort_inplace, inplace);
}
