fn main() {
    assert_eq!(
        Solution::find_substring_in_wrapround_string("a".to_string()),
        1
    );
    assert_eq!(
        Solution::find_substring_in_wrapround_string("cac".to_string()),
        2
    );
    assert_eq!(
        Solution::find_substring_in_wrapround_string("zab".to_string()),
        6
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/count-unique-substrings-of-a-string-s-present-in-a-wraparound-string/
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut ans = 0;
        let mut len = 0;
        let mut result = vec![0; 26];
        for i in 0..p.len() {
            let idx = p.chars().nth(i).unwrap() as i32 - 97;

            if i > 0 && p.chars().nth(i - 1).unwrap() as i32 != ((idx + 26 - 1) % 26 + 97) {
                len = 0;
            }
            len += 1;
            if len > result[idx as usize] {
                ans += len - result[idx as usize];
                result[idx as usize] = len;
            }
        }
        return ans;
    }
}
