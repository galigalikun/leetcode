fn main() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(Solution::search(vec![5], -5), -1);
    assert_eq!(Solution::search(vec![2, 5], 5), 1);
}

struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                left = mid + 1;
            } else if mid == 0 {
                break;
            } else {
                right = mid - 1;
            }
        }
        return -1;
    }
}
