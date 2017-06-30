extern crate primes;

use primes::eratosthenes;

fn main() {
    let x = eratosthenes(10);
    println!("{:?}", x);
}
