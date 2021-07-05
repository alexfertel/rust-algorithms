pub fn selection_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut smallest_idx = i;
        for j in i + 1..array.len() {
            if array[j] < array[smallest_idx] {
                smallest_idx = j;
            }
        }

        array.swap(i, smallest_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    sorts!(selection_sort);
}
