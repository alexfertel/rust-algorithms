pub fn heap_sort<T: Ord>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    heapify(array);

    let mut end = array.len() - 1;
    while end > 0 {
        array.swap(end, 0);
        end -= 1;
        siftdown(array, 0, end);
    }
}

fn heapify<T: Ord>(array: &mut [T]) {
    let start = (array.len() - 2) / 2;
    for i in (0..start + 1).rev() {
        siftdown(array, i, array.len() - 1);
    }
}

fn siftdown<T: Ord>(array: &mut [T], mut root: usize, end: usize) {
    while 2 * root < end {
        let child = 2 * root + 1;
        let mut swap = root;

        if array[swap] < array[child] {
            swap = child;
        }
        if child < end && array[swap] < array[child + 1] {
            swap = child + 1;
        }

        if swap == root {
            return;
        } else {
            array.swap(root, swap);
            root = swap;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::heap_sort;

    #[test]
    fn basic() {
        let mut array = [5, 4, 1, 6, 0];
        heap_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn repeated_elements() {
        let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
        heap_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn pre_sorted() {
        let mut array = [1, 2, 3, 4, 5, 6];
        heap_sort(&mut array);
        assert_sorted!(&array);
    }
}
