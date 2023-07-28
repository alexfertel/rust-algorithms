use crate::sorting::traits::{Sorter, InplaceSorter};
use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT: u64 = u64::pow(2, 32);

pub struct BogoSort;

impl BogoSort {
    fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }

        true
    }
    #[cfg(target_pointer_width = "64")]
    fn generate_index(range: usize, generator: &mut PCG32) -> usize {
        generator.get_u64() as usize % range
    }

    #[cfg(not(target_pointer_width = "64"))]
    fn generate_index(range: usize, generator: &mut PCG32) -> usize {
        generator.get_u32() as usize % range
    }

    /**
     * Fisher–Yates shuffle for generating random permutation.
     */
    fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
        for i in (1..len).rev() {
            let j = BogoSort::generate_index(i + 1, generator);
            arr.swap(i, j);
        }
    }
}

impl<T> InplaceSorter<T> for BogoSort {
    fn sort_inplace(arr: &mut [T])
    where
        T: Ord,
    {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as u64,
            Err(_) => DEFAULT,
        };

        let mut random_generator = PCG32::new_default(seed);

        let arr_length = arr.len();
        while !BogoSort::is_sorted(arr, arr_length) {
            BogoSort::permute_randomly(arr, arr_length, &mut random_generator);
        }
    }
}

impl<T> Sorter<T> for BogoSort {
    fn sort(arr: &[T]) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let mut arr_copy = arr.to_vec();
        BogoSort::sort_inplace(&mut arr_copy);
        arr_copy
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::BogoSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(BogoSort::sort, bogo_sort);
    sorting_tests!(BogoSort::sort_inplace, bogo_sort_inplace, inplace);
}
