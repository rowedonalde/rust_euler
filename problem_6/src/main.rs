fn main() {
    let mut sum_of_squares = 0;

    for i in 1..101 {
        sum_of_squares += i * i;
    }

    let gauss_sum = (100 + 1) * (100 / 2);

    let square_of_sum = gauss_sum * gauss_sum;

    println!(
        "Difference between sum of squares and square of sum of 1 to 100 is {}",
        square_of_sum - sum_of_squares);
}
