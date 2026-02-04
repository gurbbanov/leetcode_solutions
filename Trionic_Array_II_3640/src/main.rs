fn main() {}

struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();

        let mut first_bool = false;
        let mut second_bool = false;
        let mut temp = 0;
        let mut temp2 = 0;
        let mut count = 0;
        let mut max_sum = i64::MIN;
        let mut was_neg = false;

        for win in nums.windows(2) {
            if win[0] < win[1] {
                count += 1;
                if !first_bool {
                    if temp == 0 || was_neg {
                        temp = win[0] + win[1];
                    } else {
                        temp += win[1];
                    }
                    if win[0] < 0 || win[1] < 0 {
                        was_neg = true;
                    } else {
                        was_neg = false;
                    }
                } else if !second_bool {
                    second_bool = true;
                    temp += win[1];
                    max_sum = max_sum.max(temp);
                    temp2 = win[0] + win[1];
                    if win[0] < 0 || win[1] < 0 {
                        was_neg = true;
                    } else {
                        was_neg = false;
                    }
                } else {
                    temp += win[1];
                    max_sum = max_sum.max(temp);

                    if was_neg {
                        temp2 = win[0] + win[1];
                    } else {
                        temp2 += win[1];
                    }

                    if win[0] < 0 || win[1] < 0 {
                        was_neg = true;
                    } else {
                        was_neg = false;
                    }
                }
            } else if win[0] > win[1] {
                if !first_bool {
                    if count != 0 {
                        first_bool = true;
                        count = 0;
                        temp += win[1];
                    }
                } else if !second_bool {
                    temp += win[1];
                } else {
                    temp2 += win[1];
                    if win[1] < 0 {
                        was_neg = true;
                    } else {
                        was_neg = false;
                    }
                    temp = temp2;
                    temp2 = 0;
                    second_bool = false;
                }
            } else {
                first_bool = false;
                second_bool = false;
                was_neg = false;
                temp = 0;
                temp2 = 0;
                count = 0;
            }
        }

        if first_bool && second_bool {
            max_sum = max_sum.max(temp);
        }

        max_sum
    }
}
