pub trait Sort<T: Ord> {
    fn sort(arr: &[T]) -> [T];
}

pub trait SortMutable<T: Ord> {
    fn sort(arr: &mut [T]);
}
