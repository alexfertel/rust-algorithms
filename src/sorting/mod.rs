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

macro_rules! sorts {
    ($algorithm:ident) => {
        #[test]
        fn basic() {
            let mut array = [5, 4, 1, 6, 0];
            $algorithm(&mut array);
            assert_sorted!(&array);
        }

        #[test]
        fn repeated_elements() {
            let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
            $algorithm(&mut array);
            assert_sorted!(&array);
        }

        #[test]
        fn pre_sorted() {
            let mut array = [1, 2, 3, 4, 5, 6];
            $algorithm(&mut array);
            assert_sorted!(&array);
        }
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
mod insertion_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::insertion_sort::insertion_sort;

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
