fn main() {}

struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut mx = nums[0].max(nums[1]);
        let mut mx_ind = if nums[0] > nums[1] { 0 } else { 1 };
        let mut prev = nums[0].min(nums[1]);

        for ind in 2..nums.len() {
            if nums[ind] > mx {
                prev = mx;
                mx = nums[ind];
                mx_ind = ind;
            } else if nums[ind] > prev {
                prev = nums[ind];
            }
        }

        if prev * 2 <= mx {
            return mx_ind as i32;
        }

        -1
    }
}
