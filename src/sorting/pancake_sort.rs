use crate::sorting::traits::Sorter;

fn pancake_sort<T: Ord + Clone>(arr: &mut [T]) -> Vec<T>
where
    T: Ord + Clone,
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

pub struct PancakeSort;

impl<T> Sorter<T> for PancakeSort
where
    T: Ord + Copy + Clone,
{
    fn sort_inplace(arr: &mut [T]) {
        pancake_sort(arr);
    }

    fn sort(arr: &[T]) -> Vec<T> {
        pancake_sort(&mut arr.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::PancakeSort;

    sorting_tests!(PancakeSort::sort, pancake_sort);
    sorting_tests!(PancakeSort::sort_inplace, pancake_sort, inplace);
}
