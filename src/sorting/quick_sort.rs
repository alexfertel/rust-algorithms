fn _partition<T: Ord>(array: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while array[i as usize] < array[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && array[j as usize] > array[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            array.swap(i as usize, j as usize);
        }
    }
    array.swap(i as usize, pivot as usize);
    i
}

fn _quick_sort<T: Ord>(array: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _partition(array, lo, hi);
        _quick_sort(array, lo, p - 1);
        _quick_sort(array, p + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    _quick_sort(array, 0, (len - 1) as isize);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn basic() {
        let mut array = [5, 4, 1, 6, 0];
        quick_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn repeated_elements() {
        let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
        quick_sort(&mut array);
        assert_sorted!(&array);
    }

    #[test]
    fn pre_sorted() {
        let mut array = [1, 2, 3, 4, 5, 6];
        quick_sort(&mut array);
        assert_sorted!(&array);
    }
}
