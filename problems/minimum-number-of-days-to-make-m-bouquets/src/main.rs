fn main() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
}

struct Solution;
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let n = bloom_day.len();
        if (n as i32) < m * k {
            return -1;
        }
        let mut left = 1;
        let mut right = *bloom_day.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            let mut bouquets = 0;
            let mut flowers = 0;
            for i in 0..n {
                if bloom_day[i] <= mid {
                    flowers += 1;
                    if flowers == k {
                        bouquets += 1;
                        flowers = 0;
                    }
                } else {
                    flowers = 0;
                }
            }
            if bouquets < m {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left;
    }
}
