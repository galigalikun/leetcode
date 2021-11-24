fn main() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("Google".to_string()), true);
    assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
}

struct Solution {}
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut i = 0;
        // A-Z 65-90
        // a-z 97-122
        let mut upper = false;
        let mut lower = false;
        for c in word.chars() {
            let ascii = c as u8;
            if !(i == 0 && 65 <= ascii && ascii <= 90) {
                if 65 <= ascii && ascii <= 90 {
                    lower = true;
                    if upper {
                        return false;
                    }
                }
                if 97 <= ascii && ascii <= 122 {
                    upper = true;
                    if lower {
                        return false;
                    }
                }
            }
            i += 1;
        }
        return true;
    }
}
