fn main() {
    assert_eq!(Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()), 3);
    assert_eq!(Solution::repeated_string_match("a".to_string(), "aa".to_string()), 2);
}

struct Solution{}
impl Solution {
    fn indexof(s1: &str, s2: &str) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while i < s1.len() && j < s2.len() {
            if s1.as_bytes()[i] == s2.as_bytes()[j] {
                i += 1;
                j += 1;
            } else {
                i = i - j + 1;
                j = 0;
            }
        }
        if j == s2.len() {
            (i - j) as i32
        } else {
            -1
        }
    }
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return -1;
        }


        let a_index_of_b = Solution::indexof(&a, &b);
        if a_index_of_b == -1 {
            let b_index_of_a = Solution::indexof(&b, &a);
            if b_index_of_a == -1 {
                return -1;
            }
        } else {
            let b_index_of_a = Solution::indexof(&b, &a);
            if b_index_of_a == -1 {
                return -1;
            }
        }
        return -1;
    }
}
