use std::cmp::min;

fn top_down_merge<T: Copy + PartialOrd>(
    field: &mut Vec<T>,
    start: usize,
    middle: usize,
    end: usize,
    work: &mut Vec<T>,
) {
    let (mut i, mut j) = (start, middle);
    for item in work.iter_mut().take(end).skip(start) {
        if i < middle && (j >= end || field[i] <= field[j]) {
            *item = field[i];
            i += 1;
        } else {
            *item = field[j];
            j += 1;
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

/// Sorts a vector using the [mergesort algorithm](https://en.wikipedia.org/wiki/Merge_sort) in a
/// top down approach. Mergesort has an average case time complexity of `O(n log n)`.
///
/// # Examples
///
/// ```
/// use sort::merge_sort::merge_sort_top_down;
///
/// let mut data = vec![6,5,4,3,2,1];
/// merge_sort_top_down(&mut data);
/// assert_eq!(data, vec![1,2,3,4,5,6]);
/// ```
pub fn merge_sort_top_down<T: Copy + PartialOrd>(field: &mut Vec<T>) {
    let mut work: Vec<T> = field.clone();
    top_down_split_merge(&mut work, 0, field.len(), field);

}

fn copy_data<T: Copy + PartialOrd>(src: &[T], target: &mut [T], len: usize) {
    for (idx, item) in target.iter_mut().take(len).enumerate() {
        *item = src[idx];
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
    for item in &mut work.iter_mut().take(end).skip(left) {
        if i < right && (j >= end || field[i] <= field[j]) {
            *item = field[i];
            i += 1;
        } else {
            *item = field[j];
            j += 1;
        }
    }
}

/// Sorts a vector using the [mergesort algorithm](https://en.wikipedia.org/wiki/Merge_sort) in a
/// bottom up approach. Mergesort has an average case time complexity of `O(n log n)`.
///
/// # Examples
///
/// ```
/// use sort::merge_sort::merge_sort_bottom_up;
///
/// let mut data = vec![6,5,4,3,2,1];
/// merge_sort_bottom_up(&mut data);
/// assert_eq!(data, vec![1,2,3,4,5,6]);
/// ```
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
            i += 2 * width;
        }
        copy_data(&work, field, len);
        width *= 2;
    }
}
