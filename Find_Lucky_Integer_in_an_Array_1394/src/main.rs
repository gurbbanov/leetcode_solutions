fn main() {}

struct Solution;

impl Solution {
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
        let mut frequency = std::collections::HashMap::new();

        for i in arr {
            *frequency.entry(i).or_insert(0) += 1;
        }

        let mut cand = vec![];

        for i in frequency.keys() {
            if *i == frequency[i] {
                cand.push(i);
            }
        }

        if !cand.is_empty() {
            cand.sort_unstable();
            return *cand[cand.len() - 1];
        }

        -1

        // arr.sort_unstable();
        // arr.reverse();
        // let mut temp = 1;
        // let mut prev = arr[0];

        // for i in arr.into_iter().skip(1) {
        //     if i == prev {
        //         temp += 1;
        //     } else {
        //         if temp == prev {
        //             return prev;
        //         }
        //         prev = i;
        //         temp = 1;
        //     }
        // }

        // if temp == prev {
        //     return prev;
        // }

        // -1
    }
}
