fn main() {}

struct Solution {}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let mut e = Vec::new();
        let mut g = Vec::new();
        let mut p = Vec::new();
        let mut r = Vec::new();

        let length = code.len();

        let lines = vec![
            String::from("electronics"),
            String::from("grocery"),
            String::from("pharmacy"),
            String::from("restaurant"),
        ];

        for i in 0..length {
            if Self::code_check(&code[i]) && lines.contains(&business_line[i]) && is_active[i] {
                match &business_line[i] {
                    val if *val == lines[0] => e.push(code[i].clone()),
                    val if *val == lines[1] => g.push(code[i].clone()),
                    val if *val == lines[2] => p.push(code[i].clone()),
                    val if *val == lines[3] => r.push(code[i].clone()),
                    _ => panic!(),
                }
            }
        }

        e.sort();
        g.sort();
        p.sort();
        r.sort();

        result.extend(e);
        result.extend(g);
        result.extend(p);
        result.extend(r);

        result
    }

    pub fn code_check(code: &String) -> bool {
        if code.is_empty() {
            return false;
        }

        let mut code = code
            .trim_matches(|arg: char| char::is_ascii_alphanumeric(&arg))
            .chars()
            .collect::<Vec<_>>();

        code.retain(|arg| arg.is_ascii_punctuation());

        if code.is_empty() {
            return true;
        }

        code.dedup();

        code.len() == 1 && code.contains(&'_')
    }
}
