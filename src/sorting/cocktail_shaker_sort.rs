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
    T: Ord + Copy,
{
    fn sort(arr: &[T]) -> Vec<T> {
        let mut vec = arr.to_vec();
        CocktailShakerSort::sort_inplace(&mut vec);
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::{InplaceSorter, Sorter};
    use crate::sorting::CocktailShakerSort;

    sorting_tests!(CocktailShakerSort::sort, cocktail_shaker_sort);
    sorting_tests!(
        CocktailShakerSort::sort_inplace,
        cocktail_shaker_sort_inplace,
        inplace
    );
}
