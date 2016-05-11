// 2520 is the smallest number that can be divided by each of
// the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

extern crate common;

use common::math::is_prime;

fn main() {
    // Find the lowest common prime factors of 1-20. That is,
    // for each prime in 1-20, we need to find the highest
    // exponent used for that prime.

    // Each node is an array consisting of the prime and its
    // greatest occurring exponent:
    let mut primes = vec![];

    for i in 2..21 {
        if is_prime(i) {
            primes.push([i, 1]);
        } else {
            // We need to get the prime factorization of this composite.
            // For each prime factor, if the power of that factor is
            // the greatest we've seen so far for that prime, update
            // the HashMap.

            // The remaining value after each prime has been divided out:
            let mut leftover = i;
            for j in primes.iter_mut() {
                let prime = j[0];
                let mut exponent = 0;

                // For example: 100 = 2 * 50 = 2^2 * 25 = 2^2 * 5^2:
                while leftover % prime == 0 {
                    leftover = leftover / prime;
                    exponent += 1;
                }

                if exponent > j[1] {
                    j[1] = exponent;
                }
            }
        }
    }

    // Now we multiply all these greatest common prime factors
    // to get our result:
    let mut product = 1;

    for i in primes {
        let prime = i[0];
        let exponent = i[1] as u32;

        product *= prime.pow(exponent);
    }

    println!("Smallest integer that is evenly divisible from 1-20: {}", product);
}
