fn main() {
    assert_eq!(
        Solution::min_characters("aba".to_string(), "caa".to_string()),
        2
    );
    assert_eq!(
        Solution::min_characters("dabadd".to_string(), "cda".to_string()),
        3
    );
}

struct Solution;
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut a_count = vec![0; 26];
        let mut b_count = vec![0; 26];
        let mut a_max = 0;
        let mut b_max = 0;
        let mut a_sum = 0;
        let mut b_sum = 0;
        for c in a.chars() {
            let idx = c as usize - 'a' as usize;
            a_count[idx] += 1;
            a_sum += 1;
            a_max = a_max.max(a_count[idx]);
        }
        for c in b.chars() {
            let idx = c as usize - 'a' as usize;
            b_count[idx] += 1;
            b_sum += 1;
            b_max = b_max.max(b_count[idx]);
        }
        let mut ans = std::cmp::min(a_sum - a_max, b_sum - b_max);
        for i in 0..25 {
            a_count[i + 1] += a_count[i];
            b_count[i + 1] += b_count[i];
            ans = ans.min(a_sum - a_count[i] + b_count[i]);
            ans = ans.min(b_sum - b_count[i] + a_count[i]);
        }
        ans as i32
    }
}
