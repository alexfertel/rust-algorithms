pub fn counting_sort(array: &[usize]) -> Vec<usize> {
    let max = array.iter().max().unwrap_or(&0);

    let mut count = vec![0; max + 1];
    let mut output = vec![0; array.len()];

    for &element in array.iter() {
        count[element] += 1;
    }

    for i in 1..max + 1 {
        count[i] += count[i - 1];
    }

    for i in (0..array.len()).rev() {
        let j = array[i];
        count[j] -= 1;
        output[count[j]] = array[i];
    }

    output
}

#[cfg(test)]
mod tests {
    use super::counting_sort;

    #[test]
    fn basic() {
        let array = [5, 4, 1, 6, 0];
        let output = counting_sort(&array);
        assert_sorted!(&output);
    }

    #[test]
    fn repeated_elements() {
        let array = [5, 5, 1, 6, 1, 0, 2, 6];
        let output = counting_sort(&array);
        assert_sorted!(&output);
    }

    #[test]
    fn pre_sorted() {
        let array = [1, 2, 3, 4, 5, 6];
        let output = counting_sort(&array);
        assert_sorted!(&output);
    }
}
