fn main() {}

struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut sm1 = 0;
        let mut sm2 = (1..=n).sum::<i32>();

        for i in 1..n {
            sm1 += i;
            sm2 -= i - 1;

            if sm1 == sm2 {
                return i;
            }
        }

        -1
    }
}
