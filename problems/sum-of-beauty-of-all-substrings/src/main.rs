fn main() {
    assert_eq!(Solution::beauty_sum("aabcb".to_string()), 5);
    assert_eq!(Solution::beauty_sum("aabcbaa".to_string()), 17);
    assert_eq!(Solution::beauty_sum("xbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbbxbb".to_string()), 6902446);
}

struct Solution;
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut res = 0;
        for i in 0..n {
            let mut cnt = vec![0; 26];
            for j in i..n {
                cnt[(s[j] - b'a') as usize] += 1;
                let mut max = 0;
                let mut min = 100;
                for k in 0..26 {
                    if cnt[k] > 0 {
                        max = max.max(cnt[k]);
                        min = min.min(cnt[k]);
                    }
                }
                res += max - min;
            }
        }
        res
    }
}
