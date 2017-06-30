pub fn bubble_sort<T: Copy + PartialOrd>(field: &mut [T]) {
    for i in (1..field.len() + 1).rev() {
        for j in 0..i - 1 {
            if field[j] > field[j + 1] {
                field.swap(j, j + 1);
            }
        }
    }
}
