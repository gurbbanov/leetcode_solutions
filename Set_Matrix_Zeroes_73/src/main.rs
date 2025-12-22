fn main() {}

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut locations = Vec::<(usize, usize)>::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    locations.push((i, j));
                }
            }
        }

        for loc in locations {
            matrix.iter_mut().for_each(|f| f[loc.1] *= 0);
            matrix[loc.0].iter_mut().for_each(|f| *f *= 0);
        }
    }
}
