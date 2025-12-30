fn main() {}

struct Solution;

impl Solution {
    pub fn even_number_bitwise_o_rs(mut nums: Vec<i32>) -> i32 {
        nums.retain(|x| x % 2 == 0);

        if nums.len() < 1 {
            return 0;
        } else if nums.len() == 1 {
            return nums[0];
        }

        let mut result = nums[0] | nums[1];

        if nums.len() > 2 {
            for i in nums.into_iter().skip(2) {
                result |= i;
            }
        }

        result
    }
}
