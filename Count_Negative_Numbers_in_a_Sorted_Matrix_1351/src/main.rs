fn main() {}

struct Solution;

impl Solution {
    pub fn count_negatives(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|v| v.retain(|x| *x < 0));

        grid.iter().fold(0, |acc, x| acc + x.len()) as i32
    }
}
