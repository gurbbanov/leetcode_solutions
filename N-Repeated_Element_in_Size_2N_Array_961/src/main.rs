fn main() {}

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut frequency = std::collections::HashMap::new();

        for i in &nums {
            *frequency.entry(i).or_insert(0) += 1;
        }

        let cand = frequency.len() - 1;

        if cand * 2 == nums.len() {
            for (key, val) in frequency {
                if val == cand {
                    return *key;
                }
            }
        }

        0
    }
}
