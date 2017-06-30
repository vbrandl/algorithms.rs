/// Sorts an array using the [bubblesort algorithm](https://en.wikipedia.org/wiki/Bubble_sort).
/// Bubblesort has a time complexity of O(n^2).
///
/// # Examples
///
/// ```
/// let mut data = [5,7,2,6,1];
/// bubble_sort(&mut data);
/// assert_eq!(data, [1,2,5,6,7]);
/// ```
pub fn bubble_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    for i in (1..field.len() + 1).rev() {
        for j in 0..i - 1 {
            if field[j] > field[j + 1] {
                field.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut data = [2, 5, 4, 1, 6, 3, 7];
    bubble_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
}
