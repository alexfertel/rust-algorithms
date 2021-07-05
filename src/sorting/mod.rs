include!("macros.rs");

pub fn is_sorted<T>(iterator: impl IntoIterator<Item = T>) -> bool
where
    T: PartialOrd,
{
    let vector: Vec<T> = iterator.into_iter().collect();

    if vector.is_empty() {
        return true;
    }

    for i in 0..vector.len() - 1 {
        if vector[i + 1] < vector[i] {
            return false;
        }
    }

    true
}

mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod selection_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        assert_sorted!(&[] as &[isize]);
        assert_sorted!(&["a"]);
        assert_sorted!(&[1, 2, 3]);
        assert_sorted!(&[0, 1, 1]);

        assert_not_sorted!(&[1, 0]);
        assert_not_sorted!(&[2, 3, 1, -1, 5]);
    }
}
