fn main() {
    let mut accumulator = 0;

    for x in 1..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            accumulator = accumulator + x;
        }
    }

    println!("The sum is {}", accumulator);
}
