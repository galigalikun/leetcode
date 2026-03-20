fn main() {
    assert_eq!(
        Solution::make_largest_special("11011000".to_string()),
        "11100100"
    );
    assert_eq!(Solution::make_largest_special("10".to_string()), "10");
}
struct Solution {}
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut count = 0;
        let mut start = 0;
        let mut res = vec![];
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                res.push(format!(
                    "1{}0",
                    Self::make_largest_special(s[start + 1..i].to_string())
                ));
                start = i + 1;
            }
        }
        res.sort_by(|a, b| b.cmp(a));
        res.join("")
    }
}
