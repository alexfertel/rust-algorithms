// Convenience macros.

macro_rules! assert_sorted {
    ($iter:expr) => {
        assert!($crate::sorting::is_sorted($iter));
    };
}

macro_rules! assert_not_sorted {
    ($iter:expr) => {
        assert!(!$crate::sorting::is_sorted($iter));
    };
}

macro_rules! sorting_tests {
    ($sorter: expr, inplace) => {
        #[test]
        fn basic() {
            let mut array = [5, 4, 1, 6, 0];
            $sorter(&mut array);
            assert_sorted!(&array);
        }

        #[test]
        fn repeated_elements() {
            let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
            $sorter(&mut array);
            assert_sorted!(&array);
        }

        #[test]
        fn pre_sorted() {
            let mut array = [1, 2, 3, 4, 5, 6];
            $sorter(&mut array);
            assert_sorted!(&array);
        }
    };

    ($sorter: expr) => {
        #[test]
        fn basic() {
            let array = [5, 4, 1, 6, 0];
            let output = $sorter(&array);
            assert_sorted!(&output);
        }

        #[test]
        fn repeated_elements() {
            let array = [5, 5, 1, 6, 1, 0, 2, 6];
            let output = $sorter(&array);
            assert_sorted!(&output);
        }

        #[test]
        fn pre_sorted() {
            let array = [1, 2, 3, 4, 5, 6];
            let output = $sorter(&array);
            assert_sorted!(&output);
        }
    };
}
