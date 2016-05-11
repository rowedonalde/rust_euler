// A palindromic number reads the same both ways. The largest
// palindrome made from the product of two 2-digit numbers is
// 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    // Count down through all the combinations of three-digit
    // numbers until we find a palindrome:
    let mut largest = 0;
    let mut largest_i = 0;
    let mut largest_j = 0;
    'outer: for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if is_palindrome(product) && product > largest {
                largest = product;
                largest_i = i;
                largest_j = j;
            }
        }
    }

    println!(
        "Largest palindrome made from the product of two 3-digit numbers is {} x {} = {}",
        largest_i, largest_j, largest);
}

fn is_palindrome(n: i32) -> bool {
    let string_rep = n.to_string();
    let mut digits = &string_rep[..];

    // Find out if any of the mirrored digits don't match:
    while digits.len() > 1 {
        let first = digits.chars().nth(0);
        let last = digits.chars().last();

        if first != last {
            return false;
        }

        // Chop down the slice:
        digits = &digits[1..(digits.len() - 1)];
    }

    true
}
