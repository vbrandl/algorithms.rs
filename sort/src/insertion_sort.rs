/// Sorts an array using the [insertionsort
/// algorithm](https://en.wikipedia.org/wiki/Insertion_sort). Insertionsort has a time complexity
/// of O(n^2).
pub fn insertion_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    // for (idx, item) in field.iter().enumerate().skip(1) {
    for i in 1..field.len() {
        let to_insert = field[i];
        let mut j: usize = i;
        while j >= 1 && field[j - 1] > to_insert {
            field[j] = field[j - 1];
            j -= 1;
        }
        field[j] = to_insert;
    }
}

#[test]
fn test_insertion_sort() {
    let mut data = [4, 7, 2, 1, 5, 3, 6];
    insertion_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
}
