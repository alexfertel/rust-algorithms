use crate::sorting::traits::Sorter;

fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
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

pub struct CocktailShakerSort;

impl<T> Sorter<T> for CocktailShakerSort
where
    T: Ord + Copy,
{
    fn sort_inplace(arr: &mut [T]) {
        cocktail_shaker_sort(arr);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::CocktailShakerSort;

    sorting_tests!(CocktailShakerSort::sort, cocktail_shaker_sort);
    sorting_tests!(
        CocktailShakerSort::sort_inplace,
        cocktail_shaker_sort,
        inplace
    );
}
