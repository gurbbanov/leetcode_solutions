fn main() {}

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_ind = nums.len();

        for (ind, val) in nums.iter().enumerate() {
            if *val == 1 {
                if prev_ind != nums.len() {
                    if ind - prev_ind - 1 < k as usize {
                        return false;
                    }
                }
                prev_ind = ind;
            }
        }

        true
    }
}
