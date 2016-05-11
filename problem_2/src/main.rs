fn main() {
    let mut last_two = [1, 2];
    let mut accumulator = 0;

    while last_two[1] < 4000000 {
        if last_two[1] % 2 == 0 {
            println!("{} is even", last_two[1]);
            accumulator += last_two[1];
        }

        last_two = [last_two[1], last_two[0] + last_two[1]]
    }

    println!("Sum is {}", accumulator);
}
