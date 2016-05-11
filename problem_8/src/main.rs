use std::process;

fn main() {
    let bignum = "\
        73167176531330624919225119674426574742355349194934\
        96983520312774506326239578318016984801869478851843\
        85861560789112949495459501737958331952853208805511\
        12540698747158523863050715693290963295227443043557\
        66896648950445244523161731856403098711121722383113\
        62229893423380308135336276614282806444486645238749\
        30358907296290491560440772390713810515859307960866\
        70172427121883998797908792274921901699720888093776\
        65727333001053367881220235421809751254540594752243\
        52584907711670556013604839586446706324415722155397\
        53697817977846174064955149290862569321978468622482\
        83972241375657056057490261407972968652414535100474\
        82166370484403199890008895243450658541227588666881\
        16427171479924442928230863465674813919123162824586\
        17866458359124566529476545682848912883142607690042\
        24219022671055626321111109370544217506941658960408\
        07198403850962455444362981230987879927244284909188\
        84580156166097919133875499200524063689912560717606\
        05886116467109405077541002256983155200055935729725\
        71636269561882670428252483600823257530420752963450";

    let slice_len = 13usize;
    let mut i = 0;
    let mut last_good_i: Option<usize>;
    let mut biggest = 0;
    let mut last_pr = 0;
 
    while i + slice_len <= 1000 {
        match next_slice(&bignum, i, slice_len) {
            Some(good_slice_i) => {
                let good_slice = 
                    &bignum[good_slice_i..good_slice_i + slice_len];
                println!("Good slice is {} at index {}",
                    good_slice, good_slice_i);

                // If two slices are adjacent, multiply in the new number
                // and divide out the leftmost number:
                match last_good_i {
                    // First number we've seen:
                    None => {
                        biggest = last_pr = multiply_digits(good_slice);
                        last_good_i = i;
                    },
                    Some(lgi) => {
                        if lgi == 
                    },
                }

                i = good_slice_i + 1;
            },
 
            None => break,
        }
    }
}

fn next_slice(bignum: &str, start_i: usize, length: usize) -> Option<usize> {
    let mut i = start_i;

    let mut keep_searching = true;

    while keep_searching {
        keep_searching = false;
        let try_slice = &bignum[i..i + length];
        for (j, val) in try_slice.char_indices().rev() {
            if val == '0' {
                // We just found a zero, so we want to start over:
                i = i + j + 1;

                // If this would overflow the end index, confirm that
                // there are no more valid slices:
                if i + length > bignum.len() {
                    return None;
                }

                keep_searching = true;
                break;
            }
        }
    }

    Some(i)
}

fn multiply_digits(slice: &str) -> i64 {
    let mut digits: i64 = slice.parse().ok().expect("Parsing failed.");
    let mut product = 1;

    while digits > 0 {
        product *= digits % 10;
        digits = digits / 10;
    }

    product
}
