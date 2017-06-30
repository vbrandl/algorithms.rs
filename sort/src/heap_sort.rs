fn heapify<T: Copy + PartialOrd>(field: &mut Vec<T>, len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < len && field[left] > field[largest] {
        largest = left;
    }

    if right < len && field[right] > field[largest] {
        largest = right;
    }

    if largest != i {
        field.swap(i, largest);
        heapify(field, len, largest);
    }
}

/// Sorts a vector using the [heapsort algorithm](https://en.wikipedia.org/wiki/Heapsort). Heapsort
/// has an average time complexity of `O(n log n)`.
pub fn heap_sort<T: Copy + PartialOrd>(field: &mut Vec<T>) {
    let len = field.len();
    for i in (0..len / 2).rev() {
        heapify(field, len, i);
    }

    for i in (0..len).rev() {
        field.swap(0, i);
        heapify(field, i, 0);
    }
}

#[test]
fn test_heap_sort() {
    let mut data = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    heap_sort(&mut data);
    assert_eq!(data, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
