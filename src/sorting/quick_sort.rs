pub fn quick_sort<T: Ord>(array: &mut [T]) {
    match array.len() {
        0 | 1 => return,
        _ => {}
    }

    let (pivot, rest) = array.split_first_mut().expect("array is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    array.swap(0, left);

    let (left, right) = array.split_at_mut(left);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    sorting_tests!(quick_sort, inplace);
}
