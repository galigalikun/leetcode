fn main() {
    assert_eq!(Solution::remove_duplicates("abcd".to_string(), 2), "abcd");
    assert_eq!(Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3), "aa");
    assert_eq!(Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2), "ps");
}

struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        for c in s.chars() {
            if let Some((_, count)) = stack.last_mut() {
                if *count == k - 1 {
                    stack.pop();
                } else if *count == k - 2 {
                    if let Some((last, _)) = stack.last() {
                        if *last == c {
                            *count += 1;
                            continue;
                        }
                    }
                    stack.push((c, 1));
                } else {
                    if let Some((last, _)) = stack.last() {
                        if *last == c {
                            *count += 1;
                            continue;
                        }
                    }
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }
        }
        let mut s = String::new();
        for (c, count) in stack {
            for _ in 0..count {
                s.push(c);
            }
        }
        return s;
    }
}
