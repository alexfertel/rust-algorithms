use std::cmp;

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    let mut arr = arr.to_vec();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < arr.len() {
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    sorting_tests!(gnome_sort);

    #[test]
    fn odd_number_of_elements() {
        let res = gnome_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = gnome_sort(&vec![3]);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn empty() {
        let res = gnome_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }
}
