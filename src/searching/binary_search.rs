use std::cmp::Ordering;
/// ## Binary Search algorithm
/// Iterating over the array by dividing half of the array and returns the index value of the target element in the array.
/// you can pass both ascending and decending array respectively
/// 
/// Time complexcity = O(logN) where `N` is the length of the array
/// 
/// ## Arguments
/// `array` - The array where you want to find
/// 
/// `item` - The item which you want to find
/// 
/// ## Returns
/// `index` - the index value of that number if the item is in the array
/// 
/// `None` - If the item is not found in the array
///

pub fn binary_search<T: Ord>(array: &[T], item: &T) -> Option<usize> {
    // assuming the given array is ascending
    let mut is_asc = true;

    // checking if the given array contains more than 1 element or not because
    // like [2]. here there is no way to tell the elements are ascending or descending
    // like [1, 2] here you can check if the elements are ascending or descending
    if array.len() > 1 {
        // if it contains more than one elements then checking
        // if the array is ascending or descending by checking 1st and last element of the array
        // if 1st element > last element then array is decending. like [5, 2, 1]
        // if 1st element < last element then array us ascending. like [1, 2, 5]
        is_asc = array[0] < array[(array.len() - 1)];
    }

    // now taking 2 pointer (variables)
    // start is the starting index of the array
    let mut start:usize = 0;
    // end is the last index of the array
    let mut end:usize = array.len();

    // run the loop till starting index greater than or equals to end index
    while start < end {
        // taking a mid pointer and calculate the middle index based on start and last index
        let mid = start + (end - start) / 2;

        // now checking if the array is ascending or not
        if is_asc {
            // if ascending matching the target item with the middle item of the array
            match item.cmp(&array[mid]){
                // if the middle element is the target then just return the middle index
                Ordering::Equal => return Some(mid),
                // if the middle value is less than the target element then shift the end index to middle index
                Ordering::Less => { end = mid },
                // if the middle value is less than the target element then shift the start index to (middle+1) index
                Ordering::Greater => { start = mid + 1 },
            }
        }
        else {
            // if descending matching the target item with the middle item of the array
            match item.cmp(&array[mid]){
                // if the middle element is the target then just return the middle index
                Ordering::Equal => return Some(mid),
                // if the middle value is less than the target element then shift the start index to (middle+1) index
                Ordering::Less => { start = mid + 1 },
                // if the middle value is less than the target element then shift the end index to middle index
                Ordering::Greater => { end = mid },
            }
        }
    }

    // return None if item not found in the array
    None
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    /// #### Searching while array is empty
    fn empty(){
        // Searching for integer while array is empty
        // expected index = None as array is empty
        let mut index = binary_search(&vec![], &10);
        assert_eq!(index, None);

        // Searching for string while array is empty
        // expected index = None as array is empty
        index = binary_search(&vec![], &"Hello World");
        assert_eq!(index, None);
    }

    #[test]
    /// #### Searching while element not in the array
    fn not_found(){
        // Searching while element(integer) not in the array
        // expected index = None as array doesn't contain `10`
        let mut index = binary_search(&vec![1, 2, 3, 53, 100], &10);
        assert_eq!(index, None);

        // Searching while element(String) not in the array
        // expected index = None as array doesn't contain "hello World"
        index = binary_search(&vec!["A", "B", "C", "Hola"], &"Hello World");
        assert_eq!(index, None);
    }

    #[test]
    /// #### Searching while the array has one element
    fn one_element(){
        // Searching for 1 in the array of integers
        // expected index = 0 as there is only one element and the target element is also the same one element
        let mut index = binary_search(&vec![1], &1);
        assert_eq!(index, Some(0));

        // Searching for "a" in the array of strings
        // expected index = 0 as there is only one element and the target element is also the same one element
        index = binary_search(&vec!["a"], &"a");
        assert_eq!(index, Some(0));
    }

    #[test]
    /// #### Searching while the array has one element but does not contain the target element
    fn one_element_not_found(){
        // Searching for 2 in the array of integers
        // expected None as there is only one element and the target element is not the same one element
        let mut index = binary_search(&vec![1], &2);
        assert_eq!(index, None);

        // Searching for "b" in the array of strings
        // expected None as there is only one element and the target element is not the same one element
        index = binary_search(&vec!["a"], &"b");
        assert_eq!(index, None);
    }

    #[test]
    /// #### Searching while the array has ascending integer elements
    fn search_integers_asc(){
        // Declaring the test integer array with ascending elements 
        let test_array = [8, 10, 67, 87, 92, 181];

        // Searching for 10 in the array of ascending integers
        // expected index = 1 as element `10` in the 2nd position of the array
        let mut index = binary_search(&test_array, &10);
        assert_eq!(index, Some(1));

        // Searching for 87 in the array of ascending integers
        // expected index = 3 as element `87` in the 4th position of the array
        index = binary_search(&test_array, &87);
        assert_eq!(index, Some(3));

        // Searching for 181 in the array of ascending integers
        // expected index = 5 as element `181` in the 6th position of the array
        index = binary_search(&test_array, &181);
        assert_eq!(index, Some(5));
    }

    #[test]
    /// #### Searching while the array has descending integer elements
    fn search_integers_desc(){
        // Declaring the test integer array with descending elements 
        let test_array = [181, 92, 87, 67, 10, 8];

        // Searching for 10 in the array of descending integers
        // expected index = 4 as element `10` in the 5th position of the array
        let mut index = binary_search(&test_array, &10);
        assert_eq!(index, Some(4));

        // Searching for 87 in the array of descending integers
        // expected index = 2 as element `87` in the 3rd position of the array
        index = binary_search(&test_array, &87);
        assert_eq!(index, Some(2));

        // Searching for 181 in the array of descending integers
        // expected index = 0 as element `181` in the 1st position of the array
        index = binary_search(&test_array, &181);
        assert_eq!(index, Some(0));
    }

    #[test]
    /// #### Searching while the array has ascending string elements
    fn search_strings_asc(){
        // Declaring the test string array with ascending elements 
        let test_array = ["a", "b", "c", "d", "hello"];

        // Searching for "a" in the array of ascending strings
        // expected index = 0 as element `a` in the 1st position of the array
        let mut index = binary_search(&test_array, &"a");
        assert_eq!(index, Some(0));

        // Searching for "hello" in the array of ascending strings
        // expected index = 4 as element `hello` in the 5th position of the array
        index = binary_search(&test_array, &"hello");
        assert_eq!(index, Some(4));

        // Searching for "c" in the array of ascending strings
        // expected index = 2 as element `c` in the 3rd position of the array
        index = binary_search(&test_array, &"c");
        assert_eq!(index, Some(2));
    }

    #[test]
    /// #### Searching while the array has descending string elements
    fn search_strings_desc(){
        // Declaring the test string array with ascending elements 
        let test_array = ["hello", "d", "c", "b", "a"];

        // Searching for "a" in the array of descending strings
        // expected index = 4 as element `a` in the 5th position of the array
        let mut index = binary_search(&test_array, &"a");
        assert_eq!(index, Some(4));

        // Searching for "hello" in the array of descending strings
        // expected index = 0 as element `hello` in the 1st position of the array
        index = binary_search(&test_array, &"hello");
        assert_eq!(index, Some(0));

        // Searching for "c" in the array of descending strings
        // expected index = 2 as element `"c"` in the 3rd position of the array
        index = binary_search(&test_array, &"c");
        assert_eq!(index, Some(2));
    }
}