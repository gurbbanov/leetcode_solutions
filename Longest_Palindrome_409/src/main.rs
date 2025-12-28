use std::collections::HashMap;
fn main() {}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut frequency = HashMap::new();
        let mut result = 0;

        let length = s.len() as i32;

        let s = s.into_bytes();

        if s.iter().filter(|x| **x != s[0]).count() == 0 {
            return length;
        }

        for i in s {
            *frequency.entry(i).or_insert(0) += 1;
        }

        result = frequency
            .values()
            .fold(0, |result, i| result + (i - (i % 2)));

        if result != length {
            result += 1;
        }

        result
    }
}
