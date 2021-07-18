include!("macros.rs");

pub fn is_sorted<T: fmt::Debug>(iterator: impl IntoIterator<Item = T>) -> bool
where
    T: PartialOrd,
{
    let vector: Vec<T> = iterator.into_iter().collect();

    if vector.is_empty() {
        return true;
    }

    for i in 0..vector.len() - 1 {
        if vector[i + 1] < vector[i] {
            println!("This vector is not sorted: {:#?}", vector);
            return false;
        }
    }

    true
}

mod bubble_sort;
mod counting_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod selection_sort;

use std::fmt;

pub use self::bubble_sort::bubble_sort;
pub use self::counting_sort::counting_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;
pub use self::selection_sort::selection_sort;

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
