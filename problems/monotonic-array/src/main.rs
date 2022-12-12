fn main() {
    assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
    assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false);
    assert_eq!(
        Solution::is_monotonic(vec![11, 11, 9, 4, 3, 3, 3, 1, -1, -1, 3, 3, 3, 5, 5, 5]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut ans = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
            } else if nums[i] > nums[i + 1] {
                if ans == 2 {
                    return false;
                }
                ans = 1;
            } else {
                if ans == 1 {
                    return false;
                }
                ans = 2;
            }
        }
        return true;
    }
}
