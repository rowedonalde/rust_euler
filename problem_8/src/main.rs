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

    let mult_len = 13;
    let mut biggest = 0;

    // Break the bignum up into a vector of slices split apart
    // by the zero digits. If any slices are less than 13 chars
    // long, don't keep them.
    let slices: Vec<&str> = bignum.split('0').filter(|x| x.len() >= mult_len)
        .collect();

    for slice in slices.iter() {
        let slice_len = slice.len();

        // The number of shifts right we'll make:
        let shifts = (slice_len as i32) - (mult_len as i32);

        // Get seed product for this slice:
        let mut seed = multiply_digits(&slice[0..mult_len]);

        if seed > biggest {
            biggest = seed;
        }

        // Got caught up on an off-by-one error here. Since
        // we're essentially indexing from 1 (because var i is
        // how many to the right we're shifted right now) we
        // still want to include the number of shifts in the
        // iteration. Therefore, we should end with shifts + 1.
        for i in 1..(shifts as usize) + 1 {
            // To get each successive product, divide out the
            // previous leftmost digit and multiply in the new
            // rightmost digit:
            let old = &slice[i - 1..i].parse().ok()
                .expect("Parsing failed.");
            let new = &slice[i + mult_len - 1..i + mult_len].parse().ok()
                .expect("Parsing failed.");
            seed = seed / old * new;

            if seed > biggest {
                biggest = seed;
            }
        }
    }

    println!("Biggest product: {}", biggest);
}

fn multiply_digits(slice: &str) -> i64 {
    let error_text = format!("Parsing failed on {}", slice);
    let mut digits: i64 = slice.parse().ok()
        .expect(&error_text);
    let mut product = 1;

    while digits > 0 {
        product *= digits % 10;
        digits = digits / 10;
    }

    product
}
