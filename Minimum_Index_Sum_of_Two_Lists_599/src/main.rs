fn main() {}

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut frequency = std::collections::HashMap::new();
        let mut result = vec![];

        for (ind, r) in list1.iter().enumerate() {
            frequency.insert(r, ind);
        }

        let mut mn_ind = list1.len() + list2.len();

        for (ind2, rest2) in list2.iter().enumerate() {
            if let Some(ind1) = frequency.get(rest2) {
                if ind2 + ind1 == mn_ind {
                    result.push(rest2);
                } else if ind2 + ind1 < mn_ind {
                    mn_ind = ind2 + ind1;
                    result.clear();
                    result.push(rest2);
                }
            }
        }

        result.iter().map(|x| x.to_string()).collect::<Vec<_>>()
    }
}
