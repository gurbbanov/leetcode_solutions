fn main() {}

struct Solution;

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut temp = 0;
        let mut mmry = 0;

        for i in num.iter_mut().rev() {
            temp = *i + (k % 10) + mmry;
            k /= 10;

            if temp > 9 {
                mmry = 1;
                *i = temp % 10;
            } else {
                *i = temp;
                mmry = 0;
            }
        }

        if k > 0 {
            while k > 0 {
                temp = k % 10 + mmry;
                if temp > 9 {
                    mmry = 1;
                    num.insert(0, temp % 10);
                } else {
                    num.insert(0, temp);
                    mmry = 0;
                }
                k /= 10;
            }
        }
        if mmry > 0 {
            num.insert(0, mmry);
        }

        num
    }
}
