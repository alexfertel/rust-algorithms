use crate::sorting::traits::Sorter;
use std::cmp;

static MIN_MERGE: usize = 32;

fn min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn insertion_sort<T: Ord + Copy>(arr: &mut [T], left: usize, right: usize) -> &[T] {
    for i in (left + 1)..(right + 1) {
        let temp = arr[i];
        let mut j = (i - 1) as i32;

        while j >= (left as i32) && arr[j as usize] > temp {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = temp;
    }
    arr
}

fn merge<T: Default + Clone + Eq + Ord + Copy>(
    arr: &mut [T],
    l: usize,
    m: usize,
    r: usize,
) -> &[T] {
    let len1 = m - l + 1;
    let len2 = r - m;
    let mut left = vec![T::default(); len1 as usize];
    let mut right = vec![T::default(); len2 as usize];

    left[..len1].clone_from_slice(&arr[l..(len1 + l)]);

    for x in 0..len2 {
        right[x] = arr[m + 1 + x];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        arr[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < len2 {
        arr[k] = right[j];
        k += 1;
        j += 1;
    }
    arr
}

fn _tim_sort<T: Ord + Eq + Default + Clone + Copy>(arr: &mut [T], n: usize) {
    let min_run = min_run_length(MIN_MERGE) as usize;

    let mut i = 0;
    while i < n {
        insertion_sort(arr, i, cmp::min(i + MIN_MERGE - 1, n - 1));
        i += min_run;
    }

    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = cmp::min(left + 2 * size - 1, n - 1);
            if mid < right {
                merge(arr, left, mid, right);
            }

            left += 2 * size;
        }
        size *= 2;
    }
}

fn tim_sort<T: Ord + Eq + Default + Clone + Copy>(arr: &mut [T]) {
    let n = arr.len();
    _tim_sort(arr, n);
}

pub struct TimSort;

impl<T> Sorter<T> for TimSort
where
    T: Ord + Clone + Default + Eq + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        tim_sort(array);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::TimSort;

    sorting_tests!(TimSort::sort, tim_sort);
    sorting_tests!(TimSort::sort_inplace, tim_sort, inplace);
}
