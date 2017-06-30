pub fn insertion_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    for i in 1..field.len() {
        let to_insert = field[i];
        let mut j: usize = i;
        while j >= 1 && field[j - 1] > to_insert {
            field[j] = field[j - 1];
            j = j - 1;
        }
        field[j] = to_insert;
    }
}
