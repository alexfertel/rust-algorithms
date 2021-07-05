macro_rules! assert_sorted {
    ($array:expr) => {
        assert!($crate::sorting::is_sorted($array));
    };
}

macro_rules! assert_not_sorted {
    ($array:expr) => {
        assert!(!$crate::sorting::is_sorted($array));
    };
}

pub fn is_sorted<T>(array: &[T]) -> bool
where
    T: PartialOrd,
{
    if array.is_empty() {
        return true;
    }

    for i in 0..array.len() - 1 {
        if array[i] > array[i + 1] {
            return false;
        }
    }

    true
}

mod bubble_sort;

pub use self::bubble_sort::bubble_sort;

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
