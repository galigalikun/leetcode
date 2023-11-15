fn main() {
    assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    assert_eq!(Solution::smallest_divisor(vec![44, 22, 33, 11, 1], 5), 44);
}

struct Solution;
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();
        while left < right {
            let mid = (left + right) / 2;
            let sum = nums.iter().map(|&x| (x + mid - 1) / mid).sum::<i32>();
            if sum > threshold {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == right {
            return left;
        }
        return 0;
    }
}
