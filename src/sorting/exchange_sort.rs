use crate::sorting::traits::{InplaceSorter, Sorter};

// sorts through swapping the first value until it is at the right position, and repeating for all the following.
pub struct ExchangeSort;

impl<T> InplaceSorter<T> for ExchangeSort
where
    T: Ord + Clone,
{
    fn sort_inplace(arr: &mut [T]) {
        let length = arr.len();
        for number1 in 0..length {
            for number2 in (number1 + 1)..length {
                if arr[number2] < arr[number1] {
                    arr.swap(number1, number2)
                }
            }
        }
    }
}

impl<T> Sorter<T> for ExchangeSort
where
    T: Ord + Clone,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let length = arr.len();
        let mut vec = arr.to_vec();
        for number1 in 0..length {
            for number2 in (number1 + 1)..length {
                if vec[number2] < vec[number1] {
                    vec.swap(number1, number2)
                }
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::ExchangeSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(ExchangeSort::sort, exchange_sort);
    sorting_tests!(ExchangeSort::sort_inplace, exchange_sort_inplace, inplace);
}
