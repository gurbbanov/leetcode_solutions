fn main() {}

struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text = text.split(" ").collect::<Vec<_>>();
        let mut result = vec![];

        for i in text.windows(3) {
            if i[0] == first && i[1] == second {
                result.push(i[2].to_string());
            }
        }

        result
    }
}
