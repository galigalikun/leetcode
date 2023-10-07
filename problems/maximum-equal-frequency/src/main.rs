fn main() {
    assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    assert_eq!(
        Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
        13
    );
}

struct Solution;
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count: Vec<i32> = vec![0; 100001];
        let mut freq: Vec<i32> = vec![0; 100001];
        let mut max_freq: i32 = 0;
        let mut res = 0;
        for i in 0..nums.len() {
            let n = nums[i] as usize;
            count[n] += 1;
            let c = count[n] as usize;
            freq[c] += 1;
            max_freq = max_freq.max(c as i32);
            if freq[c] * c as i32 == i as i32 + 1 && i + 1 < nums.len() {
                res = i as i32 + 2;
            }
            let c = (count[n] - 1) as usize;
            freq[c] -= 1;
            if freq[c] == 0 && max_freq == c as i32 {
                max_freq -= 1;
            }
            if freq[max_freq as usize] * max_freq == (i + 1) as i32 && i + 1 < nums.len() {
                res = i as i32 + 2;
            }
        }
        return res;
    }
}
