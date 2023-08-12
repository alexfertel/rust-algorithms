use crate::sorting::traits::Sorter;

pub fn shell_sort<T: Ord + Copy>(values: &mut [T]) {
    // shell sort works by swiping the value at a given gap and decreasing the gap to 1
    fn insertion<T: Ord + Copy>(values: &mut [T], start: usize, gap: usize) {
        for i in ((start + gap)..values.len()).step_by(gap) {
            let val_current = values[i];
            let mut pos = i;
            // make swaps
            while pos >= gap && values[pos - gap] > val_current {
                values[pos] = values[pos - gap];
                pos = pos - gap;
            }
            values[pos] = val_current;
        }
    }

    let mut count_sublist = values.len() / 2; // makes gap as long as half of the array
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2; // makes gap as half of previous
    }
}

pub struct ShellSort;

impl<T> Sorter<T> for ShellSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        shell_sort(array);
    }
}

#[cfg(test)]
mod test {
    use crate::sorting::traits::Sorter;
    use crate::sorting::ShellSort;

    sorting_tests!(ShellSort::sort, shell_sort);
    sorting_tests!(ShellSort::sort_inplace, shell_sort, inplace);
}
