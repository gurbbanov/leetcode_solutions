fn main() {
}

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() == 0 {
            return (nums.len() * 2) as i32;
        }

        let mut zeros_pos = Vec::<usize>::new();
        
        if nums.len() >= 3 {
            for ind in 0..nums.len() {
                if nums[ind] == 0 {
                    zeros_pos.push(ind);
                }
            }
            if nums.iter().sum::<i32>() == 1 {
                return zeros_pos.len() as i32 ;
            }
        } else if nums.len() == 2 {
            if (nums[0] == 1) || (nums[1] == 1) {
                return 1;
            }
        } else {
            return 2;
        }

        let mut result = 0;

        for pos in zeros_pos {
            let mut direction = -1;
            let mut trys = 1;
            let mut temp = nums.clone();
            let mut ind = pos;
            while trys < 3 {
                if temp[ind] > 0 {
                    temp[ind] -= 1;
                    direction = !direction;
                    ind = if direction < 0 {ind - 1} else {ind + 1};
                } else {
                    if (ind == 0) || (ind == temp.len() - 1) {
                        trys += 1;
                        ind = pos;
                        direction = 1;
                        temp = nums.clone();
                        continue;
                    } else {
                        ind = if direction < 0 {ind - 1} else {ind + 1};
                    }
                }

                if temp.iter().sum::<i32>() == 0 {
                    result += 1;
                    trys += 1;
                    ind = pos;
                    direction = 1;
                    temp = nums.clone();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_valid_selections(vec![1,0,2,0,3]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_valid_selections(vec![2,3,4,0,4,1,0]), 0);
    }
}