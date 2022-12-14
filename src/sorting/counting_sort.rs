pub fn counting_sort<T: Keyed + Ord + Copy + Default>(array: &[T]) -> Vec<T> {
    let max: usize = array.iter().map(|item: &T| item.key()).max().unwrap_or(0);

    let mut count: Vec<usize> = vec![0; max + 1];
    let mut output: Vec<T> = vec![T::default(); array.len()];

    for &element in array.iter() {
        count[element.key()] += 1;
    }

    for i in 1..max + 1 {
        count[i] += count[i - 1];
    }

    for i in (0..array.len()).rev() {
        let j = array[i].key();
        count[j] -= 1;
        output[count[j]] = array[i];
    }

    output
}

pub trait Keyed {
    fn key(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::counting_sort;
    use super::Keyed;

    sorting_tests!(counting_sort);

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
    struct Custom {
        key: usize,
    }

    impl Keyed for Custom {
        fn key(&self) -> usize {
            self.key
        }
    }

    impl Keyed for usize {
        fn key(&self) -> usize {
            *self
        }
    }

    #[test]
    fn basic_struct() {
        let array = [
            Custom { key: 5 },
            Custom { key: 4 },
            Custom { key: 1 },
            Custom { key: 6 },
            Custom { key: 0 },
        ];

        let output = counting_sort(&array);
        assert_sorted!(&output);
    }

    #[test]
    fn repeated_elements_struct() {
        let array = [
            Custom { key: 5 },
            Custom { key: 5 },
            Custom { key: 1 },
            Custom { key: 6 },
            Custom { key: 1 },
            Custom { key: 0 },
            Custom { key: 2 },
            Custom { key: 6 },
        ];
        let output = counting_sort(&array);
        assert_sorted!(&output);
    }

    #[test]
    fn pre_sorted_struct() {
        let array = [
            Custom { key: 1 },
            Custom { key: 2 },
            Custom { key: 3 },
            Custom { key: 4 },
            Custom { key: 5 },
            Custom { key: 6 },
        ];
        let output = counting_sort(&array);
        assert_sorted!(&output);
    }
}
