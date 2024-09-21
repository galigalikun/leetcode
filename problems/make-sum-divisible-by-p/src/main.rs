fn main() {
    assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
    assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
    assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
}

struct Solution;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let p = p as i64;
        let mut sum = 0;
        for &num in &nums {
            sum += num as i64;
        }
        let target = sum % p;
        if target == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut min_len = n as i32;
        for i in 0..n {
            sum += nums[i] as i64;
            let key = (sum - target).rem_euclid(p);
            if let Some(&j) = map.get(&key) {
                min_len = min_len.min(i as i32 - j);
            }
            map.insert(sum.rem_euclid(p), i as i32);
        }
        if min_len == n as i32 {
            return -1;
        }
        min_len
    }
}
