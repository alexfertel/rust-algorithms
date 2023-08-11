use crate::sorting::traits::{InplaceSorter, Sorter};

// sorts with the minimum number of rewrites. Runs through all values in the array, placing them in their correct spots. O(n^2).
pub struct CycleSort;

impl<T> InplaceSorter<T> for CycleSort
where
    T: Ord + Clone,
{
    fn sort_inplace(arr: &mut [T]) {
        for cycle_start in 0..arr.len() {
            let mut item = arr[cycle_start].clone();
            let mut pos = cycle_start;
            for i in arr.iter().skip(cycle_start + 1) {
                if *i < item {
                    pos += 1;
                }
            }
            if pos == cycle_start {
                continue;
            }
            while item == arr[pos] {
                pos += 1;
            }
            std::mem::swap(&mut arr[pos], &mut item);
            while pos != cycle_start {
                pos = cycle_start;
                for i in arr.iter().skip(cycle_start + 1) {
                    if *i < item {
                        pos += 1;
                    }
                }
                while item == arr[pos] {
                    pos += 1;
                }
                std::mem::swap(&mut arr[pos], &mut item);
            }
        }
    }
}


impl<T> Sorter<T> for CycleSort
where
    T: Ord + Clone,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let mut vec = arr.to_vec();
        
        for cycle_start in 0..vec.len() {
            let mut item = vec[cycle_start].clone();
            let mut pos = cycle_start;
            for i in vec.iter().skip(cycle_start + 1) {
                if *i < item {
                    pos += 1;
                }
            }
            if pos == cycle_start {
                continue;
            }
            while item == vec[pos] {
                pos += 1;
            }
            std::mem::swap(&mut vec[pos], &mut item);
            while pos != cycle_start {
                pos = cycle_start;
                for i in vec.iter().skip(cycle_start + 1) {
                    if *i < item {
                        pos += 1;
                    }
                }
                while item == vec[pos] {
                    pos += 1;
                }
                std::mem::swap(&mut vec[pos], &mut item);
            }
        }

        vec
    }
}


#[cfg(test)]
mod tests {
    use crate::sorting::CycleSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(CycleSort::sort, cycle_sort);
    sorting_tests!(CycleSort::sort_inplace, cycle_sort_inplace, inplace);
}
