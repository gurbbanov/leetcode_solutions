fn main() {}

struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = nums[0].clone();

        for arr in nums.iter().skip(1) {
            result = result
                .iter()
                .filter(|x| arr.contains(x))
                .map(|x| x.to_owned())
                .collect::<Vec<i32>>();
        }

        result.sort();

        result
    }
}
