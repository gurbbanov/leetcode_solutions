fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut min_diff = i32::MAX;
        let mut result = vec![];

        for win in arr.windows(2) {
            let diff = (win[1] - win[0]).abs();
            if diff == min_diff {
                result.push(vec![win[0], win[1]]);
            } else if diff < min_diff {
                min_diff = diff;
                result.clear();
                result.push(vec![win[0], win[1]]);
            }
        }

        result
    }
}
