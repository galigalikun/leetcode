fn main() {
    assert_eq!(Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()), 3);
    assert_eq!(Solution::repeated_string_match("a".to_string(), "aa".to_string()), 2);
}

struct Solution{}
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if a.len() >= b.len() {
            if a.chars().collect::<Vec<_>>().contains(&b) {
                return 1;
            } else {
                let tmp1 = format!("{}{}", a, a);
                if tmp1.chars().collect::<Vec<_>>().contains(&b) {
                    return 2;
                } else {
                    return -1
            }
        } else {
            let num1 = b.len()/a.len();
            let num2 = b.len()%a.len();
            let mut tmp2 = a;
            if num2 != 0 {
                for i in 1..=num1 {
                    tmp2 = format!("{}{}", tmp2, a);
                }
                if tmp2.chars().collect::<vec<_>>().contains(&b) {
                    return num1 + 1;
                } else {
                    tmp2 = format!("{}{}", tmp2, a);
                    if tmp2.chars().collect::<vec<_>>().contains(&b) {
                        return num1 + 2;
                    } else {
                        return -1;
                    }
                }
            } else {
                for i in 1..num1 {
                    tmp2 = format!("{}{}", tmp2, a);
                    if tmp2.chars().collect::<vec<_>>().contains(&b) {
                        return num1;
                    } else {
                        tmp2 = format!("{}{}", tmp2, a);
                        if tmp2.chars().collect::<vec<_>>().contains(&b) {
                            return num1 + 1;
                        } else {
                            return -1;
                        }
                    }
                }
            }
        }
    }
}
