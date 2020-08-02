use std::time::Instant;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut template = strs[0].to_string();

        while template.len() > 0 {
            let mut matched_all = true;
            for s in &strs {
                if !s.starts_with(&template) {
                    matched_all = false;
                    break;
                }
            }
            if matched_all {
                return template;
            } else {
                template.pop();
            }
        }
        template
    }
}

fn main() {
    let arr = [String::from("flower"), String::from("flow"), String::from("flight")];

    let start = Instant::now();
    let result = Solution::longest_common_prefix(Vec::from(arr));
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
