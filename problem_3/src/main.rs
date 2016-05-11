// Project Euler Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

extern crate common;

use std::process;
use common::math::is_prime;

fn main() {
    let x = 600851475143;

    //let x = 1000000000;

    if is_prime(x) {
        println!(
            "{} is prime which means {} is its own largest prime factor.", x, x);
        process::exit(0);
    }

    // largest known prime factor:
    let mut lkpf: i64 = 0;

    // One of the factors of x above the square root of x:
    let mut upper_factor: i64;

    for i in 2..(((x as f64).sqrt() as i64) + 1) {
        // First check to see if i is a lower factor of x:
        if x % i == 0 {
            upper_factor = x / i;

            // Since the upper factors are counting down, we
            // know that if this upper factor is prime, it's the
            // largest prime factor of x:
            if is_prime(upper_factor) {
                //println!("The largest prime factor of {} is {}.",
                //    x, upper_factor);
                //process::exit(0);
                lkpf = upper_factor;
                break;
            }

            // Otherwise, if i is prime, we know it's the largest
            // known prime factor since the lower factors are
            // counting up:
            if is_prime(i) {
                lkpf = i;
            }
        }
    }

    println!("The largest prime factor of {} is {}.", x, lkpf);
}
