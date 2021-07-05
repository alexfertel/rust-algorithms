pub fn selection_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut smallest_idx = i;
        for j in i + 1..array.len() {
            if array[j] < array[smallest_idx] {
                smallest_idx = j;
            }
        }

        array.swap(i, smallest_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn basic() {
        let mut array = [5, 4, 1, 6, 0];
        selection_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn repeated_elements() {
        let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
        selection_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn pre_sorted() {
        let mut array = [1, 2, 3, 4, 5, 6];
        selection_sort(&mut array);
        assert_sorted!(&array);
    }
}
