use std::collections::HashSet;
fn main() {}

struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut result = HashSet::new();

        for i in nums {
            for j in i[0]..=i[1] {
                result.insert(j);
            }
        }

        result.len() as i32
    }
}
