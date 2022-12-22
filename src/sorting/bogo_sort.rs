use super::traits::SortMutable;
use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT: u64 = 4294967296;

pub struct BogoSort();

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

impl<T: Ord> SortMutable<T> for BogoSort {
    fn sort(arr: &mut [T]) {
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

#[cfg(test)]
mod tests {
    use super::super::traits::SortMutable;
    use super::BogoSort;

    sorting_tests!(BogoSort::sort, inplace);
}
