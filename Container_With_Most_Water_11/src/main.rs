fn main() {}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut mx = 0;
        let mut left_bord = 0_usize;
        let mut right_bord = height.len() - 1;
        let mut mn_side;
        let mut mx_side;
        let mut temp;

        while right_bord - left_bord > 0 {
            temp = (right_bord - left_bord) as i32;
            if height[right_bord] > height[left_bord] {
                mx_side = &mut right_bord;
                mn_side = &mut left_bord;
            } else {
                mx_side = &mut left_bord;
                mn_side = &mut right_bord;
            }

            if height[*mn_side] != 0 && height[*mx_side] != 0 {
                temp *= height[*mn_side];
                if temp > mx {
                    mx = temp;
                }
            }
            if mn_side > mx_side {
                *mn_side -= 1;
            } else {
                *mn_side += 1;
            }
        }

        mx
    }
}
