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
