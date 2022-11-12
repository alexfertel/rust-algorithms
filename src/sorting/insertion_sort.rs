// Insertion sort divides the array into sorted and unsorted parts.
// Values from the unsorted parts are placed in the correct position in the sorted part.
// Time complexity is O(N^2)
// Auxiliary space is O(1)

pub fn insertion_sort<T: Ord>(array: &mut [T]) {
    // Initialise the array.
    for i in 0..array.len() {
        let mut j = i;
        // Move elements of arr[0..i-1],
        // that are greater than key, to one
        // position ahead of their
        // current position.
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    sorting_tests!(insertion_sort, inplace);
}
