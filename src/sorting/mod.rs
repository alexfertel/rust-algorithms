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

mod bogo_sort;
mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod cycle_sort;
mod exchange_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod odd_even_sort;
mod pancake_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod stooge_sort;
mod tim_sort;
mod traits;

use std::fmt;

pub use self::bogo_sort::BogoSort;
pub use self::bubble_sort::bubble_sort;
pub use self::bucket_sort::bucket_sort;
pub use self::cocktail_shaker_sort::cocktail_shaker_sort;
pub use self::comb_sort::comb_sort;
pub use self::counting_sort::counting_sort;
pub use self::cycle_sort::cycle_sort;
pub use self::exchange_sort::exchange_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;
pub use self::odd_even_sort::odd_even_sort;
pub use self::pancake_sort::pancake_sort;
pub use self::quick_sort::quick_sort;
pub use self::radix_sort::radix_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;
pub use self::stooge_sort::stooge_sort;
pub use self::tim_sort::tim_sort;

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
