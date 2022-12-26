pub trait Sorter<T> {
    fn sort(arr: &[T]) -> [T]
    where
        T: Ord + Copy;
}

pub trait MutableSorter<T> {
    fn sort(arr: &mut [T])
    where
        T: Ord;
}
