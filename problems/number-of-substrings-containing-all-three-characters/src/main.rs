fn main() {
    assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
    assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
    assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = [0; 3];
        let mut result = 0;
        let mut j = 0;
        for (_i, c) in s.chars().enumerate() {
            count[c as usize - 'a' as usize] += 1;
            while count.iter().all(|&x| x > 0) {
                count[s.chars().nth(j).unwrap() as usize - 'a' as usize] -= 1;
                j += 1;
            }
            result += j as i32;
        }
        return result;
    }
}
