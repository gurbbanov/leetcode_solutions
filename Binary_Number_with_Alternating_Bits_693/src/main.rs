fn main() {}

struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut result = String::new();

        while n > 0 {
            if n % 2 == 0 {
                result.push_str("0");
            } else {
                result.push_str("1");
            }

            n /= 2;
        }

        !(result.contains("00") || result.contains("11"))
    }
}
