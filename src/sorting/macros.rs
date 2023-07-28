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
    ($sorter:expr, $mod_name: ident, inplace) => {
        paste::paste! {
            #[test]
            fn [< basic_ $mod_name _inplace>]() {
                let mut array = [5, 4, 1, 6, 0];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< repeated_elements_ $mod_name _inplace>]() {
                let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< pre_sorted_ $mod_name _inplace>]() {
                let mut array = [1, 2, 3, 4, 5, 6];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< descending_ $mod_name _inplace>]() {
                let mut array = [6, 5, 4, 3, 2, 1];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< empty_ $mod_name _inplace>]() {
                let mut array: [i32; 0] = [];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< one_element_ $mod_name _inplace>]() {
                let mut array = [4];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< two_elements_ $mod_name _inplace>]() {
                let mut array = [4, 1];
                $sorter(&mut array);
                assert_sorted!(&array);
            }

            #[test]
            fn [< odd_number_of_elements_ $mod_name _inplace>]() {
                let mut array = [1, 21, 5, 11, 58];
                $sorter(&mut array);
                assert_sorted!(&array);
            }
        }
    };

    ($sorter:expr, $mod_name: ident) => {
        paste::paste! {
            #[test]
            fn [< basic_ $mod_name >]() {
                let array = [5, 4, 1, 6, 0];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< repeated_elements_ $mod_name >]() {
                let array = [5, 5, 1, 6, 1, 0, 2, 6];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< pre_sorted_ $mod_name >]() {
                let array = [1, 2, 3, 4, 5, 6];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< descending_ $mod_name >]() {
                let array = [6, 5, 4, 3, 2, 1];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< empty_ $mod_name >]() {
                let array: [i32; 0] = [];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< one_element_ $mod_name >]() {
                let array = [4];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< two_elements_ $mod_name >]() {
                let array = [4, 1];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }

            #[test]
            fn [< odd_number_of_elements_ $mod_name >]() {
                let array = [1, 21, 5, 11, 58];
                let output = $sorter(&array);
                assert_sorted!(&output);
            }
        }
    };
}

