fn main() {}

struct Solution;

impl Solution {
    pub fn sum_four_divisors(mut nums: Vec<i32>) -> i32 {
        let mut divisors = std::collections::HashSet::new();
        let mut frequency = std::collections::HashMap::new();

        let mut result: i32 = 0;

        for i in nums {
            *frequency.entry(i).or_insert(0) += 1;
        }

        for (key, val) in frequency {
            let mut div = 2;
            divisors.insert(1);
            divisors.insert(key);

            while div * div <= key {
                if key % div == 0 {
                    divisors.insert(div);
                    divisors.insert(key / div);
                }

                if divisors.len() > 4 {
                    break;
                }
                div += 1;
            }

            if divisors.len() == 4 {
                result += divisors.iter().sum::<i32>() * val;
            }
            divisors.clear();
        }

        result
    }
}
