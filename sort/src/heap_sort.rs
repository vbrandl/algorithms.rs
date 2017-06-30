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
