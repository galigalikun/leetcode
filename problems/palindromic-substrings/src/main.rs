fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}

struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0;
        for i in 0..s.len() {
            ans += 1;
            for j in 1..=i {
                if &s[i - j..i + 1].to_string()
                    == &s[i - j..i + 1].chars().rev().collect::<String>()
                {
                    ans += 1;
                }
            }
        }
        return ans;
    }
}
