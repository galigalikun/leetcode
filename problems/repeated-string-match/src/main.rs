fn main() {
    // assert_eq!(Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()), 3);
    // assert_eq!(Solution::repeated_string_match("a".to_string(), "aa".to_string()), 2);
    assert_eq!(Solution::repeated_string_match("baaabbbaba".to_string(), "baaabbbababaaabbbababaaabbbababaaabbbababaaabbbababaaabbbababaaabbbababaaabbbababaaabbbababaaabbbaba".to_string()), 10);
}

// https://jpdebug.com/p/2660110
struct Solution {}
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if a.len() >= b.len() {
            if a.contains(&b) {
                return 1;
            } else {
                let tmp1 = format!("{}{}", a, a);
                if tmp1.contains(&b) {
                    return 2;
                } else {
                    return -1;
                }
            }
        } else {
            let num1 = (b.len() / a.len()) as i32;
            let num2 = (b.len() % a.len()) as i32;
            let mut tmp2 = a.clone();
            if num2 != 0 {
                for _i in 1..=num1 {
                    tmp2 = format!("{}{}", tmp2, a);
                }
                if tmp2.contains(&b) {
                    return num1 + 1;
                } else {
                    tmp2 = format!("{}{}", tmp2, a);
                    if tmp2.contains(&b) {
                        return num1 + 2;
                    } else {
                        return -1;
                    }
                }
            } else {
                for _i in 1..num1 {
                    tmp2 = format!("{}{}", tmp2, a);
                }
                if tmp2.contains(&b) {
                    return num1;
                } else {
                    tmp2 = format!("{}{}", tmp2, a);
                    if tmp2.contains(&b) {
                        return num1 + 1;
                    } else {
                        return -1;
                    }
                }
            }
        }
    }
}
