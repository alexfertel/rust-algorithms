use std::convert::TryInto;

pub fn pigeonhole_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let min = *arr.iter().min().expect("Array should not be empty");
    let max = *arr.iter().max().expect("Array should not be empty");
    let range = max - min + 1_i32; // Find range

    // Create the holes
    let mut holes: Vec<Vec<i32>> = vec![Vec::new(); range.try_into().unwrap()];

    // Fill the holes
    for &value in arr.iter() {
        holes[(value - min) as usize].push(value);
    }

    // Put the elements back into the array
    let mut index = 0;
    for hole in holes.iter() {
        for &value in hole {
            arr[index] = value;
            index += 1;
        }
    }
}

fn main() {
    let mut arr = vec![10, 10, 9, 8, 7, 7, 6, 5, 4, 3, 2, 1];
    pigeonhole_sort(&mut arr);

    println!("Sorted order is : {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pigeonhole_sort() {
        let mut arr = vec![10, 10, 9, 8, 7, 8, 6, 5, 4, -1, 3, 2, 1];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 10, 10]);
    }

    #[test]
    fn test_pigeonhole_sort_empty() {
        let mut arr: Vec<i32> = Vec::new();
        pigeonhole_sort(&mut arr);
    }

    #[test]
    fn test_pigeonhole_sort_single() {
        let mut arr = vec![1];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_pigeonhole_sort_negative() {
        let mut arr = vec![-1, -2, -3, -4, -5];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_pigeonhole_sort_duplicates() {
        let mut arr = vec![1, 1, 1, 1, 1];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_error() {
        let mut arr = vec![1, 2, 3, 4, 5];
        pigeonhole_sort(&mut arr);
        assert_ne!(arr, vec![2, 1, 3, 4, 5]);
    }

    #[test]
    fn test_big() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_big_reverse() {
        let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        pigeonhole_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
