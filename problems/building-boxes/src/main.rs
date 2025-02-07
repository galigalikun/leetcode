fn main() {
    assert_eq!(Solution::minimum_boxes(3), 3);
    assert_eq!(Solution::minimum_boxes(4), 3);
    assert_eq!(Solution::minimum_boxes(10), 6);
}

struct Solution;
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let mut n = n;
        let mut total = 0;
        let mut level = 0;
        let mut boxes = 0;
        while n > 0 {
            level += 1;
            total += level;
            n -= total;
            boxes += 1;
        }
        if n == 0 {
            return boxes;
        }
        let mut left = 0;
        let mut right = level;
        while left < right {
            let mid = left + (right - left) / 2;
            let sum = mid * (mid + 1) / 2;
            if sum == n {
                return boxes + mid;
            } else if sum < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }s
        return 0;
    }
}
