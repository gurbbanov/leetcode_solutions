fn main() {}

struct Solution {}

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut result = t.len();
        let tchars = t.chars().collect::<Vec<_>>();
        let mut ind = 0;

        for i in s.chars() {
            if i == tchars[ind] {
                result -= 1;
                ind += 1;
                if result == 0 {
                    break;
                }
            }
        }

        result as i32
    }
}
