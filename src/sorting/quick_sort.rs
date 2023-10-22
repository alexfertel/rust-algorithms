/// Sorts an array using the QuickSort algorithm.
///
/// QuickSort is a Divide and Conquer algorithm. It picks an element as a pivot and partitions
/// the given array around the picked pivot.
///
/// # Parameters
///
/// - `array`: A mutable reference to the array to be sorted.
///
/// The key process in QuickSort is a partition. The target of partitions is, given an array and an
/// element `x` of an array as the pivot, put `x` at its correct position in a sorted array and put
/// all smaller elements (smaller than `x`) before `x`, and put all greater elements (greater than `x`)
/// after `x. All this should be done in linear time.
///
/// QuickSort's time complexity is O(n*logn).
pub fn quick_sort<T: Ord>(array: &mut [T]) {
    match array.len() {
        0 | 1 => return,
        _ => {}
    }

    let (pivot, rest) = array.split_first_mut().expect("array is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    array.swap(0, left);

    let (left, right) = array.split_at_mut(left);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

/// QuickSort is a type that implements the `Sorter` trait for QuickSort.
pub struct QuickSort;

impl<T> Sorter<T> for QuickSort
where
    T: Ord + Copy,
{
    fn sort_inplace(array: &mut [T]) {
        quick_sort(array);
    }
}

// Example module organization structure
mod sorting {
    pub mod traits {
        pub trait Sorter<T> {
            fn sort_inplace(array: &mut [T]);
        }
    }

    pub mod quicksort {
        use super::traits::Sorter;

        /// Sorts an array using the QuickSort algorithm.
        pub fn quick_sort<T: Ord>(array: &mut [T]) {
            // ... (QuickSort implementation)
        }

        /// QuickSort is a type that implements the `Sorter` trait for QuickSort.
        pub struct QuickSort;

        impl<T> Sorter<T> for QuickSort
        where
            T: Ord + Copy,
        {
            fn sort_inplace(array: &mut [T]) {
                quick_sort(array);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::quicksort::QuickSort;

    // Add your unit tests here
}
