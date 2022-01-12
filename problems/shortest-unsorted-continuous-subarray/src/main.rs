fn main() {
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
}

// https://dev.to/seanpgallivan/solution-shortest-unsorted-continuous-subarray-22o2
struct Solution {}
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len() - 1;
        let mut left = -1;
        let mut right = -1;
        let mut max = nums[0];
        let mut min = nums[len];
        for i in 1..=len {
            let (a, b) = (nums[i], nums[len - i]);
            if max > a {
                right = i as i32;
            } else {
                max = a;
            }
            if b > min {
                left = i as i32;
            } else {
                min = b;
            }
        }
        return std::cmp::max(0, left + right - len as i32 + 1);
    }
}
