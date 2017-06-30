/// Implementation of the [Sieve of
/// Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes). This algorithm finds all
/// primes up to a given limit `k`.
///
/// # Examples
///
/// ```
/// use primes::eratosthenes;
/// let primes = eratosthenes(20);
/// let expected = vec![2,3,5,7,11,13,17,19];
/// assert_eq!(primes, expected);
/// ```
pub fn eratosthenes(k: usize) -> Vec<usize> {
    // index + 2 = actual number since 0 and 1 can be skipped
    let mut sieve: Vec<bool> = vec![true; k - 2];
    for idx in 2..k {
        let mut x = 2 * idx;
        while x < k {
            sieve[x - 2] = false;
            x += idx;
        }
    }

    let mut res: Vec<usize> = Vec::new();
    for (idx, item) in sieve.iter().enumerate() {
        if *item {
            res.push(idx + 2);
        }
    }
    res
}
