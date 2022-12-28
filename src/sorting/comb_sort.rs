use super::traits::MutableSorter;

pub struct CombSort;

impl<T> MutableSorter<T> for CombSort {
    fn sort(arr: &mut [T])
    where
        T: Ord,
    {
        let mut gap = arr.len();
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

#[cfg(test)]
mod tests {
    use super::CombSort;
    use super::MutableSorter;

    sorting_tests!(CombSort::sort, inplace);
}
