fn main() {
    assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
    assert_eq!(
        Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
        "j-Ih-gfE-dCba"
    );
    assert_eq!(
        Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!"
    );
}

struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut ans = vec!['\0'; s.len()];

        for c in s.chars().enumerate() {
            if !c.1.is_alphabetic() {
                ans[c.0] = c.1;
            }
        }
        let mut idx = 0;
        for c in s.chars().rev() {
            if c.is_alphabetic() {
                loop {
                    if ans[idx] == '\0' {
                        ans[idx] = c;
                        idx += 1;
                        break;
                    }
                    idx += 1;
                }
            }
        }
        return ans.iter().collect::<String>();
    }
}
