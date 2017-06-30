use std::cmp::min;

fn top_down_merge<T: Copy + PartialOrd>(
    field: &mut Vec<T>,
    start: usize,
    middle: usize,
    end: usize,
    work: &mut Vec<T>,
) {
    let (mut i, mut j) = (start, middle);
    for k in start..end {
        if i < middle && (j >= end || field[i] <= field[j]) {
            work[k] = field[i];
            i = i + 1;
        } else {
            work[k] = field[j];
            j = j + 1;
        }
    }
}

fn top_down_split_merge<T: Copy + PartialOrd>(
    work: &mut Vec<T>,
    start: usize,
    end: usize,
    field: &mut Vec<T>,
) {
    if end - start < 2 {
        return;
    }

    let middle: usize = (end + start) / 2;
    top_down_split_merge(field, start, middle, work);
    top_down_split_merge(field, middle, end, work);

    top_down_merge(work, start, middle, end, field);
}

pub fn merge_sort_top_down<T: Copy + PartialOrd>(field: &mut Vec<T>) {
    let mut work: Vec<T> = field.clone();
    top_down_split_merge(&mut work, 0, field.len(), field);

}

fn copy_data<T: Copy + PartialOrd>(src: &Vec<T>, target: &mut Vec<T>, len: usize) {
    for i in 0..len {
        target[i] = src[i];
    }
}

fn bottom_up_merge<T: Copy + PartialOrd>(
    field: &mut Vec<T>,
    left: usize,
    right: usize,
    end: usize,
    work: &mut Vec<T>,
) {
    let (mut i, mut j) = (left, right);
    for k in left..end {
        if i < right && (j >= end || field[i] <= field[j]) {
            work[k] = field[i];
            i = i + 1;
        } else {
            work[k] = field[j];
            j = j + 1;
        }
    }
}

pub fn merge_sort_bottom_up<T: Copy + PartialOrd>(field: &mut Vec<T>) {
    let mut work: Vec<T> = field.clone();
    let mut width = 1;
    let len = field.len();
    while width < len {
        let mut i = 0;
        while i < len {
            bottom_up_merge(
                field,
                i,
                min(i + width, len),
                min(i + 2 * width, len),
                &mut work,
            );
            i = i + 2 * width;
        }
        copy_data(&work, field, len);
        width = width * 2;
    }
}
