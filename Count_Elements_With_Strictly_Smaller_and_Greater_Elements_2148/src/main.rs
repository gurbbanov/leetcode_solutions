use std::collections::HashSet;

fn main() {}

struct Solution;

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut has_bigger = false;
        let mut has_smaller = false;
        let mut result = HashSet::new();

        'a: for i in 0..nums.len() {
            'b: for j in 0..nums.len() {
                if i != j {
                    if nums[i] > nums[j] {
                        has_smaller = true;
                    } else if nums[i] < nums[j] {
                        has_bigger = true;
                    }

                    if has_bigger && has_smaller {
                        result.insert(i);
                        break 'b;
                    }
                }
            }
            has_bigger = false;
            has_smaller = false;
        }

        result.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
    }
}
