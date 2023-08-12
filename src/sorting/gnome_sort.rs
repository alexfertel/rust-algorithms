use crate::sorting::traits::Sorter;

fn gnome_sort<T: Ord>(arr: &mut [T]) {
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
}

pub struct GnomeSort;

impl<T> Sorter<T> for GnomeSort
where
    T: Ord + Copy,
{
    fn sort_inplace(arr: &mut [T]) {
        gnome_sort(arr);
    }
}


#[cfg(test)]
mod tests {
    use crate::sorting::traits::Sorter;
    use crate::sorting::GnomeSort;

    sorting_tests!(GnomeSort::sort, gnome_sort);
    sorting_tests!(GnomeSort::sort_inplace, gnome_sort, inplace);
}
