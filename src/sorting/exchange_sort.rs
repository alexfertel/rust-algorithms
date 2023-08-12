use crate::sorting::traits::Sorter;

fn exchange_sort<T: Ord>(arr: &mut [T]) {
    let length = arr.len();
    for number1 in 0..length {
        for number2 in (number1 + 1)..length {
            if arr[number2] < arr[number1] {
                arr.swap(number1, number2)
            }
        }
    }
}

// sorts through swapping the first value until it is at the right position, and repeating for all the following.
pub struct ExchangeSort;

impl<T> Sorter<T> for ExchangeSort
where
    T: Ord + Copy,
{
    fn sort_inplace(arr: &mut [T]) {
        exchange_sort(arr);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::ExchangeSort;

    sorting_tests!(ExchangeSort::sort, exchange_sort);
    sorting_tests!(ExchangeSort::sort_inplace, exchange_sort, inplace);
}
