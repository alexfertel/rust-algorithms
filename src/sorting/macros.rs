// Convenience macros.

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

// Macros for automatically producing tests.

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
