use crate::sorting::traits::{Sorter, InplaceSorter};
use crate::sorting::insertion_sort::InsertionSort;

/// Sort a slice using bucket sort algorithm.
///
/// Time complexity is `O(n + k)` on average, where `n` is the number of elements,
/// `k` is the number of buckets used in process.
///
/// Space complexity is `O(n + k)`, as it sorts not in-place.
pub struct BucketSort;

impl<T> InplaceSorter<T> for BucketSort
where
    T: Ord + Copy + Into<usize>,
{
    fn sort_inplace(arr: &mut [T]) {
        if arr.is_empty() {
            return;
        }

        let max = *arr.iter().max().unwrap();
        let len = arr.len();
        let mut buckets = vec![vec![]; len + 1];

        for x in arr.iter() {
            buckets[len * (*x).into() / max.into()].push(*x);
        }

        for bucket in buckets.iter_mut() {
            InsertionSort::sort_inplace(bucket);
        }

        let mut i = 0;
        for bucket in buckets {
            for x in bucket {
                arr[i] = x;
                i += 1;
            }
        }
    }
}


impl<T> Sorter<T> for BucketSort
where
    T: Ord + Copy + Into<usize>,
{
    fn sort(arr: &[T]) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let mut arr = arr.to_vec();
        BucketSort::sort_inplace(&mut arr);
        arr
    }
}


#[cfg(test)]
mod tests {
    use crate::sorting::BucketSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};
}
