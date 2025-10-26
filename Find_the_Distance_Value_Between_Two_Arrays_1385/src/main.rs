fn main() {
}

struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut result = 0;
        let mut temp = 0;

        for i in arr1 {
            for j in &arr2 {
                if (i - j).abs() <= d {
                    temp += 1;
                }
            }
            if temp == 0 {
                result += 1;
            }
            temp = 0;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_the_distance_value(vec![1,4,2,3], vec![-4,-3,6,10,20,30], 3), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_the_distance_value(vec![2,1,100,3], vec![-5,-2,10,-3,7], 6), 1);
    }
}