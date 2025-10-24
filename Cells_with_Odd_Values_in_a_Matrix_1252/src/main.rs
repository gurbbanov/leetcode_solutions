fn main() {
    Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]);
}

struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![0; n as usize]; m as usize];
        let mut result = 0;
        for i in indices {
            matrix[i[0] as usize]
                .iter_mut()
                .for_each(|x| *x += 1);

            matrix
                .iter_mut()
                .for_each(|x| x[i[1] as usize] += 1);
        }

        for i in matrix {
            for j in i {
                if j % 2 != 0 {
                    result += 1
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
    fn case1() {
        assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}