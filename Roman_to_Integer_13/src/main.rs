fn main() {}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut d1;
        let mut d2;
        let mut result = 0;
        let mut length = 0;
        let mut skip_next = false;

        for win in s.chars().collect::<Vec<_>>().windows(2) {
            if skip_next {
                skip_next = false;
                continue;
            }

            d1 = Self::get_digit(win[0]);
            d2 = Self::get_digit(win[1]);
            if d1 < d2 {
                result += d2 - d1;
                length += 2;
                skip_next = true;
            } else {
                result += d1;
                length += 1;
            }
        }

        if length == s.len() {
            return result;
        } else {
            result += Self::get_digit(s.chars().last().unwrap());
            result
        }
    }

    pub fn get_digit(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}
