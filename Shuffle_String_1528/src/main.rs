fn main() {}

struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s: Vec<_> = s.chars().collect();
        let mut result = String::from("");

        for i in 0..indices.len() as i32 {
            result.push(s[indices.iter().position(|pos| *pos == i).unwrap()]);
        }

        result

        // alternative solution

        // let mut s = s.chars().collect::<Vec<_>>();

        // let mut s = s.iter().zip(indices).collect::<Vec<_>>();
        // s.sort_by_key(|x| x.1);
        // let mut result = vec![];
        // for i in s {
        //     result.push(i.0);
        // }

        // result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::restore_string(String::from("codeleet"), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            String::from("leetcode")
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::restore_string(String::from("abc"), vec![0, 1, 2]),
            String::from("abc")
        );
    }
}
