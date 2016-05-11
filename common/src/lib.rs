pub mod math {
    pub fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        } else if n <= 3 {
            return true;
        } else if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i: i64 = 5;

        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }

        true
    }
}
