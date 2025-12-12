use std::collections::{HashMap, HashSet};

fn main() {}

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        if length < 4 {
            if nums[0] + nums[1] + nums[2] == 0 {
                return vec![vec![nums[0], nums[1], nums[2]]];
            }
        }

        let mut cl = nums.clone();
        cl.dedup();
        if cl.len() == 1 && cl[0] == 0 {
            return vec![vec![0; 3]];
        }

        let mut nums_map = HashMap::new();
        for i in &nums {
            nums_map
                .entry(i)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut temp;
        let mut cand;
        let mut result = HashSet::new();

        for i in 0..length {
            for j in i + 1..length {
                temp = (nums[i] + nums[j]) * -1;
                if nums[i] == nums[j] && nums[j] == temp {
                    if *nums_map.get(&temp).unwrap() >= 3 {
                        result.insert(vec![temp; 3]);
                    }
                } else {
                    if nums_map.contains_key(&temp) {
                        if (nums[i] == temp) || (nums[j] == temp) {
                            if nums[i] != nums[j] {
                                if *nums_map.get(&temp).unwrap() > 1 {
                                    cand = vec![nums[i], nums[j], temp];
                                    cand.sort();
                                    result.insert(cand);
                                }
                            }
                        } else {
                            cand = vec![nums[i], nums[j], temp];
                            cand.sort();
                            result.insert(cand);
                        }
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

// impl Solution {
//     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let length = nums.len();
//         if length < 4 {
//             if nums[0] + nums[1] + nums[2] == 0 {
//                 return vec![vec![nums[0], nums[1], nums[2]]];
//             }
//         }

//         // let mut cl = nums.clone();
//         // cl.dedup();
//         // if cl.len() == 1 {
//         //     return vec![vec![]];
//         // }

//         nums.sort_unstable();
//         // nums.dedup();
//         let mut result = HashSet::new();
//         let mut temp;
//         let mut pos;

//         for i in 0..length {
//             'b: for j in i + 1..length {
//                 if nums.contains(&((nums[i] + nums[j]) * -1)) {
//                     pos = nums
//                         .iter()
//                         .position(|x| *x == (nums[i] + nums[j]) * -1)
//                         .unwrap();
//                     if nums[pos - 1] == nums[pos] || nums[pos + 1] == nums[pos] {
//                         temp = vec![nums[i], nums[j], (nums[i] + nums[j]) * -1];
//                         temp.sort_unstable();
//                         result.insert(temp);
//                     }
//                 }
//                 if (nums[i] + nums[j]).abs() > nums[0].abs()
//                     || (nums[i] + nums[j]).abs() > nums[length - 1].abs()
//                 {
//                     break 'b;
//                 }
//                 // else {
//                 // for k in (length / 2..length).rev() {
//                 // for k in j + 1..length {
// println!("{} {} {}", nums[i], nums[j], nums[k]);
//                 //     temp = nums[i] + nums[j] + nums[k];
//                 //     // if nums[i] + nums[j] + nums[k] == 0 {
//                 //     if temp == 0 {
//                 //         // result.insert(temp);
//                 //         result.insert(vec![nums[i], nums[j], nums[k]]);
//                 //     }

//                 //     if temp < 0 {
//                 //         break;
//                 //     }
//                 // }
//                 // }
//             }
//         }

//         result.into_iter().collect::<Vec<_>>()
//     }
// }
