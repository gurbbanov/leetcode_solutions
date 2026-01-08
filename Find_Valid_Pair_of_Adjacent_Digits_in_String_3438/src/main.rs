fn main() {}

struct Solution;

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut frequency = std::collections::HashMap::new();

        for i in s.chars() {
            *frequency.entry(i).or_insert(0) += 1;
        }

        for win in s.chars().collect::<Vec<_>>().windows(2) {
            if win[0] != win[1]
                && ((frequency[&win[0]] == win[0].to_digit(10).unwrap())
                    && (win[1].to_digit(10).unwrap() == frequency[&win[1]]))
            {
                return format!("{}{}", win[0], win[1]);
            }
        }

        String::new()
    }
}
