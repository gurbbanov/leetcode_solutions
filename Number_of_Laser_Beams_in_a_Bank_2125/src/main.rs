fn main() {
}

struct Solution;

impl Solution {
    pub fn number_of_beams(mut bank: Vec<String>) -> i32 {
        if bank.is_empty() || bank.len() < 2 {
            return 0;
        }

        for row in &mut bank {
            row.retain(|beam| beam == '1');
        }

        bank.retain(|row| !row.is_empty());

        if bank.is_empty() {
            return 0;
        } else if bank.len() == 2 {
            return (bank[0].len() * bank[1].len()) as i32;
        }

        let mut result = 0;
        for ind in 0..bank.len() - 1 {
            result += bank[ind].len() * bank[ind + 1].len();
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::number_of_beams(vec![String::from("011001"),String::from("000000"),String::from("010100"), String::from("001000")]), 8);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::number_of_beams(vec![String::from("000"),String::from("111"),String::from("000")]), 0);
    }
}