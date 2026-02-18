fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        let mut rev_bin = Vec::with_capacity(32);

        while n > 0 {
            rev_bin.push(n % 2);

            n /= 2;
        }

        rev_bin.resize_with(32, || 0);

        let mut result = 0;

        for i in rev_bin {
            result = (result * 2) + i;
        }

        result
    }
}
