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

    //SECOND SOLUTION
    pub fn minimum_cost_1(nums: Vec<i32>) -> i32 {
        if nums.len() == 3 {
            return nums.iter().sum::<i32>();
        }

        let mut ind = 1;
        let mut mn = i32::MAX;
        let mut mn_ind = 1;
        let mut prev_mn = i32::MAX;

        while ind < nums.len() {
            if nums[ind] < mn {
                mn = nums[ind];
                mn_ind = ind;
            }

            ind += 1;
        }

        ind = 1;
        while ind < nums.len() {
            if ind == mn_ind {
                ind += 1;
                continue;
            }

            if nums[ind] < prev_mn {
                prev_mn = nums[ind];
            }

            ind += 1;
        }

        nums[0] + mn + prev_mn
    }
}
