fn main() {}

struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        for i in &nums {
            if i.abs() < result.abs() {
                result = *i;
            }
        }

        if result < 0 {
            if nums.contains(&result.abs()) {
                return result.abs();
            }
        }

        result
    }
}
