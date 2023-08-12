use crate::sorting::traits::Sorter;

fn counting_sort<T: Ord + Copy + Default + Into<usize>>(arr: &[T]) -> Vec<T> {
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

    output
}

pub struct CountingSort;

impl<T> Sorter<T> for CountingSort
where
    T: Ord + Copy + Default + Into<usize>,
{
    fn sort_inplace(arr: &mut [T]) {
        let output = counting_sort(arr);
        arr.copy_from_slice(&output);
    }

    fn sort(arr: &[T]) -> Vec<T> {
        counting_sort(arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::CountingSort;

    sorting_tests!(CountingSort::sort, counting_sort);
    sorting_tests!(CountingSort::sort_inplace, counting_sort, inplace);
}
