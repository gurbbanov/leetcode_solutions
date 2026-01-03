fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        let mut result = nums[0];

        for window in nums.windows(k as usize) {
            if window[0] - window[k as usize - 1] < result {
                result = window[0] - window[k as usize - 1];
            }
        }

        result
    }
}
