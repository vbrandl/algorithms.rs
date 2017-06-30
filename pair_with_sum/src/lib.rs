extern crate sort;

use std::ops::Add;

/// Given `field[] = {a_0, ..., a_n-1}, s` where `a_i` and `s` are signed integers and `n` is a unsigned
/// integer (say `n` signed integers), this algorithm returns `true` if there exists a pair of
/// elements `a_i, a_j` where `s = a_i + a_j` and `i != j`, else it returns `false`. This
/// implementation has a time complexity of `O(n log n)`. First the field is sorted using
/// heapsort (`O(n log n)`), then the field is iterated form left and right at the same time to
/// find a sum (`O(n)`).
///
/// # Examples
///
/// ```
/// use pair_with_sum::pair_with_sum;
///
/// let mut data = [6,3,5,2,1];
/// assert!(pair_with_sum(&mut data, &11));
/// let mut data = [6,3,5,2,1];
/// assert!(!pair_with_sum(&mut data, &12));
/// ```
pub fn pair_with_sum<T>(field: &mut [T], sum: &<T as Add>::Output) -> bool
where
    T: Copy + PartialOrd + Add,
    <T as Add>::Output: PartialOrd,
{
    use sort::heap_sort::heap_sort;

    heap_sort(field);
    let mut left = 0;
    let mut right = field.len() - 1;
    while left < right {
        if field[left] + field[right] == *sum {
            return true;
        } else if field[left] + field[right] < *sum {
            left += 1;
        } else {
            right += 1;
        }
    }
    false
}

/// Given `field[] = {a_0, ..., a_n-1}, s` where `a_i` and `s` are signed integers and `n` is a unsigned
/// integer (say `n` signed integers), this algorithm returns `true` if there exists a pair of
/// elements `a_i, a_j` where `s = a_i + a_j` and `i != j`, else it returns `false`. This is a naiv
/// implementation with a time complexity of `O(n^2)`
///
/// # Examples
///
/// ```
/// use pair_with_sum::pair_with_sum_naiv;
///
/// let data = [6,3,5,2,1];
/// assert!(pair_with_sum_naive(&data, &11));
/// assert!(!pair_with_sum_naive(&data, &12));
/// ```
pub fn pair_with_sum_naive<T>(field: &[T], sum: &<T as Add>::Output) -> bool
where
    T: Add + Copy,
    <T as Add>::Output: PartialOrd,
{
    for (i, item) in field.iter().enumerate().take(field.len() - 1) {
        for item2 in field.iter().skip(i + 1) {
            if *item + *item2 == *sum {
                return true;
            }
        }
    }
    false
}
