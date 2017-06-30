/// Sorts an array using the [selectionsort
/// algorithm](https://en.wikipedia.org/wiki/Selectionsort). Selectionsort has a time complexity of
/// `O(n^2)`.
///
/// # Examples
///
/// ```
/// use sort::selection_sort::selection_sort;
///
/// let mut data = [6,5,4,3,2,1];
/// selection_sort(&mut data);
/// assert_eq!(data, [1,2,3,4,5,6]);
/// ```
pub fn selection_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    for left in 0..field.len() {
        let mut min = left;
        for (i, item) in field.iter().enumerate().skip(left + 1) {
            if *item < field[min] {
                min = i;
            }
        }
        field.swap(min, left);
    }
}
