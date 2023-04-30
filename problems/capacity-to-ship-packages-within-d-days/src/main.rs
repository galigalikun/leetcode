fn main() {
    assert_eq!(
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
        15
    );
    assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
}

struct Solution;
impl Solution {
    fn check(weights: &Vec<i32>, days: i32, capacity: i32) -> bool {
        let mut need = 1;
        let mut cur = 0;
        for i in 0..weights.len() {
            if cur + weights[i] > capacity {
                need += 1;
                cur = 0;
            }
            cur += weights[i];
        }
        return need <= days;
    }
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for i in 0..weights.len() {
            left = left.max(weights[i]);
            right += weights[i];
        }
        while left < right {
            let mid = (left + right) / 2;
            if Solution::check(&weights, days, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}
