// It sorts the array by repeatedly comparing the
// adjacent elements and swapping them if they are
// in the wrong order.
// Time complexity is O(N^2)
// Auxiliary space is O(1)

pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        // Last i elements are already in place.
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    sorting_tests!(bubble_sort, inplace);
}
