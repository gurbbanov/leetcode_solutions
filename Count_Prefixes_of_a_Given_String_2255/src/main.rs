fn main() {}

struct Solution;

impl Solution {
    // pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    //     let mut result = 0;

    //     for i in words {
    //         if s.starts_with(&i) {
    //             result += 1;
    //         }
    //     }

    //     result
    // }

    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut result = 0;

        for i in words {
            if i.len() > s.len() {
                continue;
            }

            if s[..i.len()] == i {
                result += 1;
            }
        }

        result
    }
}
