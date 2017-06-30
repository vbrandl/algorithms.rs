# Pair with sum

The exercise for this one was to create an algorithm with runtime complexity `O(n log n)` that solves the following
problem:

Input: `A[] = {a_0, ..., a_n-1}, s` where `a_i` and `s` are signed integers and `n` is an unsigned integer (so an array
with `n` elements).

Output: `true` if there are two elements `a_i` and `a_j` where `s = a_i + a_j` and `i != j`, else `false`

# Naive approach

The naive approach to this problem (without taking care about the runtime complexity) was to have two for loops that
iterate over the array and build a sum of each two elements from the array and check against `s`. It looks something
like this:

``` Rust
for (i, item) in field.iter().enumerate().take(field.len() - 1) {
    for item2 in field.iter().skip(i + 1) {
        if *item + *item2 == *sum {
            return true;
        }
    }
}
false
```

That approach has a time complexity of `O(n^2)` so there had to be a better way.

# The solution

The solution was to sort the array in the first step using a fast algorithm like heap sort or merge sort. Both
algorithms have a complexity of `O(n log n)` so that's still valid. After that I had to find two elements from the
array, that fulfill my given condition without exceeding a complexity of `O(n log n)`. Given a sorted array, this is
pretty simple since we can iterate over the array from both sides at the same time and add the both elements, our
iterators point to. We iterate as long as the left iterator is less than the right iterator and check if the current sum
is less than `sum` or not. If it is less, the left iterator is incremented so the sum gets bigger. Else the right
iterator is decremented to get a smaller sum. The algorithm looks like this:

``` Rust
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
```

Finding the sum is done in `O(n)` so the complete complexity is `O( (n log n) + n )` which is effectively `O(n log n)`.
