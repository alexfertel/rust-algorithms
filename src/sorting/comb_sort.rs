use crate::sorting::traits::Sorter;

fn comb_sort<T: Ord>(arr: &mut [T]) {
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

pub struct CombSort;

impl<T> Sorter<T> for CombSort
where
    T: Ord + Copy,
{
    fn sort_inplace(arr: &mut [T]) {
        comb_sort(arr);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::CombSort;

    sorting_tests!(CombSort::sort, comb_sort);
    sorting_tests!(CombSort::sort_inplace, comb_sort, inplace);
}
