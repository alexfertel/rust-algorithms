pub fn merge_sort<T: Ord + Copy>(array: &[T]) -> Vec<T> {
    if array.len() < 2 {
        return array.to_vec();
    }

    let middle = array.len() / 2;

    let mut left = merge_sort(&array[..middle]);
    let mut right = merge_sort(&array[middle..]);

    merge(&mut left, &mut right)
}

fn merge<T: Ord + Copy>(left: &mut Vec<T>, right: &mut Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for _ in 0..left.len() + right.len() {
        if left.is_empty() {
            result.append(right);
            break;
        } else if right.is_empty() {
            result.append(left);
            break;
        } else if left[0] <= right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn basic() {
        let array = [5, 4, 1, 6, 0];
        let result = merge_sort(&array);
        assert_sorted!(result);
    }

    #[test]
    fn repeated_elements() {
        let mut array = [5, 5, 1, 6, 1, 0, 2, 6];
        let result = merge_sort(&mut array);
        assert_sorted!(result);
    }

    #[test]
    fn pre_sorted() {
        let mut array = [1, 2, 3, 4, 5, 6];
        let result = merge_sort(&mut array);
        assert_sorted!(result);
    }
}
