fn main() {
    for (a, b) in [(1, 2), (4, 1), (2, 2), (3, 3), (0, 2)] {
        let s = Solution::str_without3a3b(a, b);
        assert_eq!(s.chars().filter(|&c| c == 'a').count(), a as usize);
        assert_eq!(s.chars().filter(|&c| c == 'b').count(), b as usize);
        assert!(!s.contains("aaa"));
        assert!(!s.contains("bbb"));
    }
}

struct Solution;
impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut a = a as usize;
        let mut b = b as usize;
        let mut result = String::new();

        while a > 0 || b > 0 {
            if a > b {
                if a > 1 && b > 0 {
                    result.push_str("aab");
                    a -= 2;
                    b -= 1;
                } else if a > 1 {
                    result.push_str("aa");
                    a -= 2;
                } else {
                    result.push('a');
                    a -= 1;
                }
            } else if b > a {
                if b > 1 && a > 0 {
                    result.push_str("bba");
                    b -= 2;
                    a -= 1;
                } else if b > 1 {
                    result.push_str("bb");
                    b -= 2;
                } else {
                    result.push('b');
                    b -= 1;
                }
            } else if a > 0 {
                result.push_str("ab");
                a -= 1;
                b -= 1;
            } else {
                break;
            }
        }

        result
    }
}
