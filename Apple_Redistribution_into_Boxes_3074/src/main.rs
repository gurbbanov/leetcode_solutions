fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut apple = apple.iter().sum::<i32>();
        let mut count = 0;

        if apple == capacity.iter().sum::<i32>() {
            return capacity.len() as i32;
        }

        capacity.sort();
        // capacity.reverse();
        let mut ind = capacity.len() - 1;

        while apple > 0 {
            count += 1;
            apple -= capacity[ind];
            ind -= 1;
        }

        count
    }
}
