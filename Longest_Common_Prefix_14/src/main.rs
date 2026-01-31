fn main() {}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.first().unwrap().to_owned();
        }

        let first = &strs[0];
        let second = &strs[1];

        let mut border = 0;
        let mut count = 0;
        let mut prev = "";

        for (x, y) in first.chars().zip(second.chars()) {
            if x == y {
                border += 1;
            } else {
                break;
            }
        }

        if border == 0 {
            return String::new();
        } else {
            count += 2;
        }

        prev = &first[..border];
        border = 0;

        for st in strs.iter().skip(2) {
            for (x, y) in st.chars().zip(prev.chars()) {
                if x == y {
                    border += 1;
                } else {
                    break;
                }
            }

            if border != 0 {
                count += 1;
                prev = prev.min(&first[..border]);
            }
            border = 0;
        }

        if count == strs.len() {
            return prev.to_string();
        }

        String::new()
    }
}
