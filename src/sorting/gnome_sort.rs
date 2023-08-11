pub fn gnome_sort<T>(arr: &mut [T])
where
    T: cmp::PartialOrd,
{
    let mut i = 1;

    while i < arr.len() {
        if i == 0 || arr[i - 1] <= arr[i] {
            i += 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    sorting_tests!(gnome_sort);

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec!["d", "a", "c", "e", "b"];
        gnome_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let mut arr = vec![3];
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![3]);
    }

    #[test]
    fn empty() {
        let mut arr: Vec<u8> = Vec::new();
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
