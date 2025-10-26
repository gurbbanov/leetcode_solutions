fn main() {
}

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<char> = words[0].chars().collect();
        let mut temp: Vec<char>;

        for i in 1..words.len() {
            temp = words[i].chars().collect();

            result.retain(|chr| if let Some(pos) = temp.iter().position(|c| c == chr) {
                temp.remove(pos);
                true
            } else {
                false
            });
        }

        let result: Vec<String> = result.iter().map(|c| c.to_string()).collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::common_chars(vec![String::from("bella"), String::from("label"), String::from("roller")]), vec!["e", "l", "l"])
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::common_chars(vec![String::from("cool") ,String::from("lock"),String::from("cook")]), vec!["c", "o"]);
    }
}