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

    sorting_tests!(merge_sort);
}
