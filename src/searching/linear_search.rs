/// ## Linear Search algorithm.
/// Iterating over the array linearly and returns the index value of the target element in the array.
///
/// Time complexcity = O(N/2) where `N` is the length of the array.
///
/// ## Arguments
/// `array` - The array where you want to find.
///
/// `item` - The item which you want to find.
///
/// ## Returns
/// `index` - the index value of that number if the item is in the array.
///
/// `None` - If the item is not found in the array.
///

pub fn linear_search<T: PartialEq>(array: &[T], item: &T) -> Option<usize> {
    // Time complexcity = O(N).
    // iterating over the given array using `iter` method and for extracting the index using `enumerate` method.
    // for (index, element) in array.iter().enumerate(){
    // checking if the target item is the same array element
    //     if item == element {
    // return the index number of the the array as an option
    //         return Some(index)
    //     }
    // }

    /*
         0  1  2  3   4   5
        [1, 2, 9, 11, 68, 92] taget = 9
        i                 i not hit   (start = 0, end = 6-1-0 = 5)
            i          i    not hit   (start = 1, end = 6-1-1 = 4)
               i   i        hit(start)(start = 2, end = 6-1-2 = 3)
    */

    // Time complexcity = O(N/2).
    // iterating over the array till half of the array.
    for i in 0..(array.len() / 2) {
        // searching in left half of the array.
        // comparing array[i] with target item.
        if array[i] == *item {
            // if found then return the index.
            return Some(i);
        }
        // searching in the right half of the array.
        // comparing array[array,len() - 1 - i] with target item.
        else if array[array.len() - 1 - i] == *item {
            // if found then return the index.
            return Some(array.len() - 1 - i);
        }
    }
    // return None if item not found in the array.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// #### Searching while array is empty.
    fn empty() {
        // Searching for integer while array is empty.
        // expected index = None as array is empty.
        let index = linear_search(&vec![], &2);
        assert_eq!(index, None);

        // Searching for strings while array is empty.
        // expected index = None as array is empty.
        let index = linear_search(&vec![], &"Hello");
        assert_eq!(index, None);
    }

    #[test]
    /// #### Searching while element not in the array.
    fn not_found() {
        // Searching while element(String) not in the array.
        // expected index = None as array doesn't contain "d".
        let index = linear_search(&vec!["a", "b", "c", "Hello"], &"d");
        assert_eq!(index, None);

        // Searching while element(integer) not in the array.
        // expected index = None as array doesn't contain 5.
        let index = linear_search(&vec![1, 2, 3, 100, 7239], &5);
        assert_eq!(index, None);
    }

    #[test]
    /// #### Searching for strings.
    fn search_strings() {
        // Defining the test string array.
        let test_array = vec!["a", "b", "c", "Hello"];

        // Searching for "Hello" in the array.
        // expected index = 3.
        let index = linear_search(&test_array, &"Hello");
        assert_eq!(index, Some(3));

        // Searching for "c" in the array.
        // expected index = 2.
        let index = linear_search(&test_array, &"c");
        assert_eq!(index, Some(2));

        // Searching for "a" in the array.
        // expected index = 0.
        let index = linear_search(&test_array, &"a");
        assert_eq!(index, Some(0));
    }

    #[test]
    /// #### Searching for integers.
    fn search_integers() {
        // Defining the test integer array.
        let test_array = vec![1, 2, 5, 735, 72, 63];

        // Searching for `735` in the array.
        // expected index = 3.
        let mut index = linear_search(&test_array, &735);
        assert_eq!(index, Some(3));

        // Searching for `63` in the array.
        // expected index = 5.
        index = linear_search(&test_array, &63);
        assert_eq!(index, Some(5));

        // Searching for `1` in the array.
        // expected index = 0.
        index = linear_search(&test_array, &1);
        assert_eq!(index, Some(0));
    }
}
