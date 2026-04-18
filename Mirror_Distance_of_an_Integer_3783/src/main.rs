fn main() {}

struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        return (n - Self::reverse(n)).abs();
    }

    pub fn reverse(n: i32) -> i32 {
        let mut n = n;

        let mut res = 0;
        while n > 0 {
            res = res * 10 + n % 10;
            n /= 10;
        }

        res += n;

        return res;
    }
}
