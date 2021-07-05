pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut array = [5, 4, 123, 5, 5462, 0];
        bubble_sort(&mut array);
        assert_sorted!(&array);
    }
}
