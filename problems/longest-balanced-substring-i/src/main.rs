fn main() {
    assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
    assert_eq!(Solution::longest_balanced("zzabccy".to_string()), 4);
    assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut count = [0; 28];
        let mut result = 0;
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
            if count[c as usize - 'a' as usize] > 0 && count[c as usize - 'a' as usize + 1] > 0 {
                result = result
                    .max(count[c as usize - 'a' as usize] + count[c as usize - 'a' as usize + 1]);
            }
        }
        result
    }
}
