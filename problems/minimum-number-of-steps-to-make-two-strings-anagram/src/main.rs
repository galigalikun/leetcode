fn main() {
    assert_eq!(Solution::min_steps("bab".to_string(), "aba".to_string()), 1);
    assert_eq!(
        Solution::min_steps("leetcode".to_string(), "practice".to_string()),
        5
    );
    assert_eq!(
        Solution::min_steps("anagram".to_string(), "mangaar".to_string()),
        0
    );
}

struct Solution;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut t = t.chars().collect::<Vec<char>>();
        s.sort();
        t.sort();
        println!("{:?} {:?}", s, t);
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else if s[i] < t[j] {
                i += 1;
                count += 1;
            } else {
                j += 1;
                count += 1;
            }
        }
        if i < s.len() {
            count += s.len() - i;
        }
        if j < t.len() {
            count += t.len() - j;
        }
        return count as i32 / 2;
    }
}
