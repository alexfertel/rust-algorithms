pub trait Sorter<T> {
    fn sort(arr: &[T]) -> Vec<T>
    where
        T: Ord + Copy;
}


pub trait InplaceSorter<T> {
    fn sort_inplace(arr: &mut [T])
    where
        T: Ord + Copy;
}

