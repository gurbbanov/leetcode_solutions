fn main() {}

struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut length = grid.len();
        let mut result = vec![vec![0; length - 2]; length - 2];

        let mut local_max = 0;
        let mut row_ind = 0;
        let mut column_ind = 0;

        while (row_ind + 3 <= length) && (column_ind + 3 <= length) {
            for vc in &grid[row_ind..row_ind + 3] {
                for val in &vc[column_ind..column_ind + 3] {
                    if *val > local_max {
                        local_max = *val;
                    }
                }
            }
            result[row_ind][column_ind] = local_max.clone();
            local_max = 0;

            column_ind += 1;
            if column_ind == length - 2 {
                column_ind = 0;
                row_ind += 1;
            }
        }

        result
    }
}
