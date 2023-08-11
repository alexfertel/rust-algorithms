use crate::sorting::traits::{InplaceSorter, Sorter};


pub struct CountingSort;


impl<T> InplaceSorter<T> for CountingSort
where
    T: Into<usize> + Ord + Copy + Default
{
    fn sort_inplace(arr: &mut [T]) {
        let max: usize = arr.iter().map(|item: &T| (*item).into()).max().unwrap_or(0);

        let mut count: Vec<usize> = vec![0; max + 1];
        let mut output: Vec<T> = vec![T::default(); arr.len()];

        for &element in arr.iter() {
            count[element.into()] += 1;
        }

        for i in 1..max + 1 {
            count[i] += count[i - 1];
        }

        for i in (0..arr.len()).rev() {
            let j = arr[i].into();
            count[j] -= 1;
            output[count[j]] = arr[i];
        }

        arr.copy_from_slice(&output);
    }
}

impl<T> Sorter<T> for CountingSort
where
    T: Into<usize> + Ord + Copy + Default,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let mut arr = arr.to_vec();
        CountingSort::sort_inplace(&mut arr);
        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::CountingSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(CountingSort::sort, counting_sort);
    //sorting_tests!(CountingSort::sort_inplace, counting_sort_inplace, inplace);
}
