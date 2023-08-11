use crate::sorting::traits::{InplaceSorter, Sorter};


pub struct CocktailShakerSort;

impl<T> InplaceSorter<T> for CocktailShakerSort
where
    T: Ord,
{
    fn sort_inplace(arr: &mut [T]) {
        let len = arr.len();

        if len == 0 {
            return;
        }

        loop {
            let mut swapped = false;

            for i in 0..(len - 1).clamp(0, len) {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }

            swapped = false;

            for i in (0..(len - 1).clamp(0, len)).rev() {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }
}

impl<T> Sorter<T> for CocktailShakerSort
where
    T: Ord + Clone,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let len = arr.len();

        if len == 0 {
            return vec![];
        }

        let mut vec = arr.to_vec();

        loop {
            let mut swapped = false;

            for i in 0..(len - 1).clamp(0, len) {
                if vec[i] > vec[i + 1] {
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }

            swapped = false;

            for i in (0..(len - 1).clamp(0, len)).rev() {
                if vec[i] > vec[i + 1] {
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::CocktailShakerSort;
    use crate::sorting::traits::{InplaceSorter, Sorter};

    sorting_tests!(CocktailShakerSort::sort, cocktail_shaker_sort);
    sorting_tests!(CocktailShakerSort::sort_inplace, cocktail_shaker_sort_inplace, inplace);
}
