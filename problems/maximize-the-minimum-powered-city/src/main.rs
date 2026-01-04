fn main() {
    assert_eq!(Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2), 5);
    assert_eq!(Solution::max_power(vec![4, 4, 4, 4], 0, 3), 4);
}

struct Solution;
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let (mut left, mut right) = (0, 1_000_000_000);
        while left < right {
            let mid = left + (right - left + 1) / 2;
            let mut needed = 0i64;
            let mut added = vec![0i64; n + 1];
            let mut current_add = 0i64;
            let mut power = 0i64;
            for i in 0..n {
                if i as i32 - r - 1 >= 0 {
                    current_add -= added[i - r as usize - 1];
                }
                power += stations[i] as i64 + current_add;
                if power < mid as i64 {
                    let diff = mid as i64 - power;
                    needed += diff;
                    if needed > k as i64 {
                        break;
                    }
                    current_add += diff;
                    power += diff;
                    if i + 2 * r as usize + 1 < n {
                        added[i + 2 * r as usize + 1] += diff;
                    }
                }
            }
            if needed <= k as i64 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        return left as i64;
    }
}
