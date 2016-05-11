extern crate common;

use common::math::is_prime;

fn main() {
    // The number of primes we've found:
    let mut n = 0;

    // The number we're looking at now:
    let mut i = 1;

    while n < 10001 {
        // We want to increment at the beginning so that we have
        // the right number on hand when we get to the 10001st prime:
        i += 1;
        if is_prime(i) {
            n += 1;
        }
    }

    println!("The 10001st prime is {}", i);
}
