fn main() {
    assert_eq!(Solution::minimum_size(vec![9], 2), 3);
    assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
}

struct Solution;
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();
        for _ in 0..100 {
            let mid = (left + right) / 2;
            let mut count = 0;
            for &num in &nums {
                count += (num - 1) / mid;
            }
            if count > max_operations {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
