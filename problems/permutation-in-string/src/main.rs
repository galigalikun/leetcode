fn main() {
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
        false
    );
    assert_eq!(
        Solution::check_inclusion("abc".to_string(), "bbbca".to_string()),
        true
    );
}

// https://just4once.gitbooks.io/leetcode-notes/content/leetcode/two-pointers/567-permutation-in-string.html
struct Solution {}
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = vec![0; 128];
        for c in s1.chars() {
            map[c as usize] += 1;
        }
        let mut count = s1.len();
        let (mut left, mut right) = (0, 0);
        while right < s2.len() {
            if map[s2.chars().nth(right).unwrap() as usize] > 0 {
                count -= 1;
            }
            map[s2.chars().nth(right).unwrap() as usize] -= 1;
            right += 1;

            while count == 0 {
                if (right - left) == s1.len() {
                    return true;
                }
                map[s2.chars().nth(left).unwrap() as usize] += 1;
                if map[s2.chars().nth(left).unwrap() as usize] > 0 {
                    count += 1;
                }
                left += 1;
            }
        }
        return false;
    }
}
