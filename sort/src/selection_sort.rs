pub fn selection_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    for left in 0..field.len() {
        let mut min = left;
        for i in left + 1..field.len() {
            if field[i] < field[min] {
                min = i;
            }
        }
        field.swap(min, left);
    }
}
