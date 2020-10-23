fn main() {
    println!("aa {}", Solution::length_of_longest_substring("dvdf".to_string()))
}

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut stack = vec![];
        let mut c = 0;
        for i in s.as_str().chars() {
            if let Some(p) = stack.iter().position(|&x| x == i) {
                if stack.len() > c {
                    c = stack.len();
                }
                stack.drain(0..p+1);
                stack.push(i);
            } else {
                stack.push(i);
            }
        }
        if stack.len() > c {
            c = stack.len();
        }
        return c as i32;
    }
}
