fn main() {}

struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut lgt_dist = 0;
        let mut temp = 0;

        let mut prev = n % 2;
        n /= 2;

        if prev == 1 {
            temp += 1;
        }

        while n > 0 {
            if n % 2 == 1 {
                if prev == 1 || temp > 0 {
                    lgt_dist = lgt_dist.max(temp);
                    temp = 1;
                } else {
                    temp += 1;
                }
            } else {
                if temp > 0 {
                    temp += 1;
                }
            }

            prev = n % 2;
            n /= 2;
        }

        lgt_dist
    }
}
