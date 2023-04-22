fn main() {
    assert_eq!(Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2), 6);
    assert_eq!(Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3), 10);
}

struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max = 0;
        let mut count = 0;
        while right < nums.len() {
            if nums[right] == 0 {
                count += 1;
            }
            while count > k {
                if nums[left] == 0 {
                    count -= 1;
                }
                left += 1;
            }
            max = std::cmp::max(max, right - left + 1);
            right += 1;
        }
        max as i32
    }
}
