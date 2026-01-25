fn main() {}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = std::collections::HashMap::new();

        for (ind, i) in nums.iter().enumerate() {
            if let Some(sb) = result.get(&(target - i)) {
                return vec![ind as i32, *sb as i32];
            };

            result.insert(i, ind);
        }

        vec![]
    }
}
