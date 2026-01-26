fn main() {}

struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        //0 ms SOLUTION
        if x < 0 {
            return false;
        } else if x < 10 {
            return true;
        }

        let mut digits = vec![];
        let mut x2 = x;

        while x2 > 0 {
            digits.push(x2 % 10);
            x2 /= 10;
        }

        if (x / 10_i32.pow((digits.len() - 1) as u32) == 1) && (x % 10 == 0) {
            return false;
        }

        let length = (digits.len() / 2) + (digits.len() % 2);

        let mut cand = 0;
        let mut counter = 10_i32.pow(length as u32);

        for i in &digits[..length] {
            counter /= 10;
            cand += i * counter;
        }

        cand == x / 10_i32.pow((digits.len() / 2) as u32)

        //SECOND SOLUTION WITH REVERSED SLICE (6 ms)
        // if x < 0 {
        //     return false;
        // } else if x < 10 {
        //     return true;
        // }

        // let mut digits = vec![];

        // while x > 0 {
        //     digits.push(x % 10);
        //     x /= 10;
        // }

        // let length = digits.len();

        // digits[..((length / 2) + (length % 2))]
        //     .iter()
        //     .rev()
        //     .collect::<Vec<_>>()
        //     == digits[((length / 2))..]
        //         .iter()
        //         .collect::<Vec<_>>()
    }
}
