use crate::sorting::traits::Sorter;

fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;

        for i in (1..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        for i in (0..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

pub struct OddEvenSort;

impl<T> Sorter<T> for OddEvenSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        odd_even_sort(array);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::OddEvenSort;

    sorting_tests!(OddEvenSort::sort, odd_even_sort);
    sorting_tests!(OddEvenSort::sort_inplace, odd_even_sort, inplace);
}
