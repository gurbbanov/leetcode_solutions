fn main() {}

struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut dp = s.chars().collect::<Vec<_>>().clone();
        dp.dedup();
        if dp.len() == 1 {
            return s.len() as i32;
        }

        let mut temp = vec![];
        let mut max_length = 1;

        for i in s.chars() {
            if temp.is_empty() {
                temp.push(i);
                println!("{:?}", temp);
            } else {
                if temp[0] == i {
                    temp.push(i);
                    println!("{:?}", temp);
                } else {
                    if temp.len() > max_length {
                        max_length = temp.len();
                        println!("{:?}", temp);
                    }
                    temp.clear();
                    temp.push(i);
                }
            }
        }
        if temp.len() > max_length {
            max_length = temp.len();
        }

        max_length as i32
    }
}

// impl Solution {
//     pub fn max_power(s: String) -> i32 {
//         let chrs: Vec<char> = s.chars().collect();
//         let mut start: Option<usize> = None;
//         let mut end: Option<usize> = None;
//         let mut max_length = 1;
//         let mut tmp;

//         for i in 0..chrs.len() - 1 {
//             if chrs[i] == chrs[i + 1] {
//                 if start == None {
//                     start = Some(i);
//                 }
//             } else {
//                 if start != None {
//                     end = Some(i);
//                     tmp = end.unwrap() - start.unwrap() + 1;
//                     if tmp > max_length {
//                         max_length = tmp;
//                     }

//                     start = None;
//                     end = None;
//                 }
//             }
//         }

//         max_length as i32
//     }
// }
