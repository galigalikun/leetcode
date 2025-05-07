fn main() {
    assert_eq!(Solution::make_string_sorted("cba".to_string()), 5);
    assert_eq!(Solution::make_string_sorted("aabaa".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = 0;
        let mut fact = vec![1; s.len() + 1];
        for i in 1..=s.len() {
            fact[i] = fact[i - 1] * i as i32;
        }
        let mut count = vec![0; 26];
        for c in s.iter() {
            count[*c as usize - 'a' as usize] += 1;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            let mut less = 0;
            for i in 0..s[left] as usize - 'a' as usize {
                if count[i] > 0 {
                    less += 1;
                }
            }
            ans += less * fact[right - left];
            count[s[left] as usize - 'a' as usize] -= 1;
            if count[s[left] as usize - 'a' as usize] == 0 {
                right -= 1;
            }
            left += 1;
        }
        let mut total = 1;
        for i in 0..26 {
            if count[i] > 1 {
                total *= fact[count[i]];
            }
        }
        ans -= total;
        ans += 1;
        ans %= 1_000_000_007;
        ans
    }
}
