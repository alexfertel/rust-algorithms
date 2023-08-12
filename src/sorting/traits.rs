pub trait Sorter<T: Ord + Copy> {
    fn sort_inplace(arr: &mut [T]);

    fn sort(arr: &[T]) -> Vec<T> {
        let mut arr = arr.to_vec();
        Self::sort_inplace(&mut arr);
        arr
    }
}

