pub fn bitonic_sort(up: bool, x: &mut [i32]) {
    if x.len() <= 1 {
        return;
    }

    let mid = x.len() / 2;
    let (first, second) = x.split_at_mut(mid);
    bitonic_sort(true, first);
    bitonic_sort(false, second);

    bitonic_merge(up, x);
}

pub struct BitonicSort;

fn bitonic_merge(up: bool, x: &mut [i32]) {
    if x.len() <= 1 {
        return;
    }

    let mid = x.len() / 2;
    for i in 0..mid {
        if up == (x[i] > x[mid + i]) {
            x.swap(i, mid + i);
        }
    }

    let (first, second) = x.split_at_mut(mid);
    bitonic_merge(up, first);
    bitonic_merge(up, second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_sort() {
        let mut numbers = vec![10, 30, 11, 20, 4, 330, 21, 110];
        bitonic_sort(true, &mut numbers);
        assert_eq!(numbers, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn test_bitonic_sort_empty() {
        let mut numbers = vec![];
        bitonic_sort(true, &mut numbers);
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_bitonic_sort_one_element() {
        let mut numbers = vec![10];
        bitonic_sort(true, &mut numbers);
        assert_eq!(numbers, vec![10]);
    }

    #[test]
    fn test_bitonic_sort_two_elements() {
        let mut numbers = vec![10, 30];
        bitonic_sort(true, &mut numbers);
        assert_eq!(numbers, vec![10, 30]);
    }

    #[test]
    fn test_error_bitonic_sort() {
        let mut numbers = vec![10, 30, 11, 20, 4, 330, 21, 110];
        bitonic_sort(true, &mut numbers);
        assert_ne!(numbers, vec![10, 4, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn test_bitonic_merge_empty() {
        let mut numbers = vec![];
        bitonic_merge(true, &mut numbers);
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_bitonic_merge_one_element() {
        let mut numbers = vec![10];
        bitonic_merge(true, &mut numbers);
        assert_eq!(numbers, vec![10]);
    }
}
