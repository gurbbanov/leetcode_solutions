fn main() {}

struct Solution;

impl Solution {
    pub fn is_prefix_string(mut s: String, mut words: Vec<String>) -> bool {
        let mut s = s.as_str();
        for i in words {
            if !s.starts_with(i.as_str()) {
                return false;
            } else {
                // s = s.replacen(i.as_str(), "", 1);
                s = s.strip_prefix(i.as_str()).unwrap();
            }

            if s.is_empty() {
                return true;
            }
        }

        false
    }
}
