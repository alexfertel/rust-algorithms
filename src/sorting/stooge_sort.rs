use crate::sorting::traits::{InplaceSorter, Sorter};

pub struct StoogeSort;

impl<T> InplaceSorter<T> for StoogeSort
where
    T: Ord,
{
    fn sort_inplace(array: &mut [T]) {
        stooge_sort(array);
    }
}

impl<T> Sorter<T> for StoogeSort
where
    T: Ord + Copy,
{
    fn sort(array: &[T]) -> Vec<T> {
        let mut vec = array.to_vec();
        stooge_sort(&mut vec);
        vec
    }
}

fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    if start + 1 >= end {
        return;
    }

    let k = (end - start + 1) / 3;

    _stooge_sort(arr, start, end - k);
    _stooge_sort(arr, start + k, end);
    _stooge_sort(arr, start, end - k);
}

fn stooge_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    _stooge_sort(arr, 0, len - 1);
}

#[cfg(test)]
mod test {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::StoogeSort;

    sorting_tests!(StoogeSort::sort, stooge_sort);
    sorting_tests!(StoogeSort::sort_inplace, stooge_sort_inplace, inplace);
}
