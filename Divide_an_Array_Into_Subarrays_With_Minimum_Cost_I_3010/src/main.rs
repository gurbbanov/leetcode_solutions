fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        if nums.len() == 3 {
            return nums.iter().sum::<i32>();
        }

        let mut freqs = std::collections::HashMap::new();

        for i in &nums[1..] {
            *freqs.entry(i).or_insert(0) += 1;
        }

        let mut result = nums[0];
        let temp = *freqs.keys().min().unwrap();
        result += temp;
        *freqs.get_mut(temp).unwrap() += -1;
        if freqs[temp] == 0 {
            freqs.remove(temp);
        }
        result += *freqs.keys().min().unwrap();

        result
    }
}
