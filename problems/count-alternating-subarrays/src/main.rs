fn main() {
    assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
    assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
}

struct Solution;
impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count = 0i64;
        let mut length = 1;
        for i in 1..n {
            if nums[i] != nums[i - 1] {
                length += 1;
            } else {
                count += (length * (length + 1) / 2) as i64;
                length = 1;
            }
        }
        count += (length * (length + 1) / 2) as i64;
        return count;
    }
}
