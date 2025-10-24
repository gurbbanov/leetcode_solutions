fn main() { }

struct Solution;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut first_candidate;
        let mut second_candidate;
        let len = nums.len();
        for ind1 in 0..len - 2 {
            for ind2 in ind1 + 1..len - 1 {
                if ind1 != ind2 {
                    first_candidate = vec![nums[ind1], nums[ind1 + 1]];
                    second_candidate = vec![nums[ind2], nums[ind2 + 1]];

                    if Self::subarray_sum(&first_candidate) == Self::subarray_sum(&second_candidate) {
                        return true;
                    }
                } 
            }
        }
        false
    }

    pub fn subarray_sum(subarray: &Vec<i32>) -> i32 {
        subarray[0] + subarray[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_subarrays(vec![4,2,4]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_subarrays(vec![1,2,3,4,5]), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_subarrays(vec![0,0,0]), true);
    }
}