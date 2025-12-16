fn main() {}

struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let length = nums.len();

        if length < 4 || nums[0] > nums[1] {
            return false;
        }

        let mut ind = 0;

        while nums[ind] < nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return false;
            }
        }

        while nums[ind] > nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return false;
            }
        }

        while nums[ind] < nums[ind + 1] {
            ind += 1;
            if ind == length - 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_trionic(vec![2, 1, 3]), false);
    }
}
