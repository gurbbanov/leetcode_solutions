use std::collections::HashMap;

fn main() {}

struct Solution;

impl Solution {
    pub fn frequency_sort(mut num: Vec<i32>) -> Vec<i32> {
        let mut frequency = HashMap::new();

        num.iter()
            .for_each(|&i| *frequency.entry(i).or_insert(0) += 1);

        num.sort_by_key(|x| (frequency[x], -(*x)));

        num
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
            vec![1, 3, 3, 2, 2]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
