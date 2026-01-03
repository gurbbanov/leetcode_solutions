fn main() {}

struct Solution;

impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        let mut bn = vec![];

        while n >= 0 {
            if n % 2 == 0 {
                bn.push("1");
            } else {
                bn.push("0");
            }
            n /= 2;

            if n == 0 {
                break;
            }
        }

        bn.reverse();

        i32::from_str_radix(bn.concat().as_str(), 2).unwrap()
    }
}
