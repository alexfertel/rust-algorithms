pub fn insertion_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    sorts!(insertion_sort);
}
