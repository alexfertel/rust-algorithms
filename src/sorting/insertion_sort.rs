pub fn insertion_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn basic() {
        let mut array = [5, 4, 1, 6, 0];
        insertion_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn repeated_elements() {
        let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
        insertion_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn pre_sorted() {
        let mut array = [1, 2, 3, 4, 5, 6];
        insertion_sort(&mut array);
        assert_sorted!(&array);
    }
}
