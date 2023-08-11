use crate::sorting::traits::{InplaceSorter, Sorter};

pub struct CombSort;

impl<T> InplaceSorter<T> for CombSort {
    fn sort_inplace(arr: &mut [T])
    where
        T: Ord,
    {
        let mut gap = arr.len();
        if gap <= 1 {
            return;
        }

        let shrink = 1.3;
        let mut sorted = false;

        while !sorted {
            gap = (gap as f32 / shrink).floor() as usize;
            if gap <= 1 {
                gap = 1;
                sorted = true;
            }
            for i in 0..arr.len() - gap {
                let j = i + gap;
                if arr[i] > arr[j] {
                    arr.swap(i, j);
                    sorted = false;
                }
            }
        }
    }
}

impl<T> Sorter<T> for CombSort
where
    T: Ord + Copy,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let mut vec = arr.to_vec();
        CombSort::sort_inplace(&mut vec);
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::CombSort;

    sorting_tests!(CombSort::sort, comb_sort);
    sorting_tests!(CombSort::sort_inplace, comb_sort_inplace, inplace);
}
