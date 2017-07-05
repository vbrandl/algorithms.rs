/// Intuitive implementation of the max partial sum problem.
///
/// Runtime complexity: `O(n^3)`
///
/// # Examples
///
/// ```
/// use partial_sum::max_partial_sum_intuitive;
///
/// let field = [-13,25,34,12,-3,7,-87,28,-77,11];
/// assert_eq!(max_partial_sum_intuitive(&field), 75);
/// ```
pub fn max_partial_sum_intuitive(field: &[i32]) -> i32 {
    let mut max = i32::min_value();
    let len = field.len();
    for i in 0..len {
        for j in i..len {
            let mut sum = 0;
            for item in field.iter().skip(i).take(j) {
                sum += *item;
            }
            if sum > max {
                max = sum;
            }
        }
    }
    max
}

/// Slightly better implementation of the max partial sum problem.
///
/// Runtime complexity: `O(n^2)`
///
/// # Examples
///
/// ```
/// use partial_sum::max_partial_sum_n_2;
///
/// let field = [-13,25,34,12,-3,7,-87,28,-77,11];
/// assert_eq!(max_partial_sum_n_2(&field), 75);
/// ```
pub fn max_partial_sum_n_2(field: &[i32]) -> i32 {
    let mut max = i32::min_value();
    let len = field.len();
    for i in 0..len {
        let mut sum = 0;
        for item in field.iter().skip(i) {
            sum += *item;
            if sum > max {
                max = sum;
            }
        }
    }
    max
}

/// Optimal implementation of the max partial sum problem.
///
/// Runtime complexity: `O(n)`
///
/// # Examples
///
/// ```
/// use partial_sum::max_partial_sum;
///
/// let field = [-13,25,34,12,-3,7,-87,28,-77,11];
/// assert_eq!(max_partial_sum(&field), 75);
/// ```
pub fn max_partial_sum(field: &[i32]) -> i32 {
    let mut max = i32::min_value();
    let mut curr_sum = 0;
    for item in field {
        let s = curr_sum + *item;
        if s > *item {
            curr_sum = s;
        } else {
            curr_sum = *item;
        }
        if curr_sum > max {
            max = curr_sum;
        }
    }
    max
}
