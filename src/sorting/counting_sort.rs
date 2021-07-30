use std::collections::HashMap;
use std::usize;

pub fn counting_sort<T: Keyed + Ord + Copy>(array: &[T]) -> Vec<T> {
    let mut new_hashmap: HashMap<usize, &T> = HashMap::new();

    for element in array {
        new_hashmap.insert(element.key(), element);
    }

    let mut usize_array: Vec<usize> = Vec::new();

    for element in array {
        usize_array.push(element.key());
    }

    let max = usize_array.iter().max().unwrap_or(&0);
    let mut count: Vec<usize> = vec![0; max + 1];
    let mut output: Vec<T> = vec![array[0]; array.len()];

    for &element in usize_array.iter() {
        count[element] += 1;
    }

    for i in 1..max + 1 {
        count[i] += count[i - 1];
    }

    for i in (0..usize_array.len()).rev() {
        let j = usize_array[i];
        count[j] -= 1;
        output[count[j]] = **new_hashmap.get(&usize_array[i]).unwrap();
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

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    fn basic() {
        let array = [5, 4, 1, 6, 0];
        let output = counting_sort(&array);
        assert_sorted!(&output);
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
    fn repeated_elements() {
        let array = [5, 5, 1, 6, 1, 0, 2, 6];
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
    fn pre_sorted() {
        let array = [1, 2, 3, 4, 5, 6];
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
