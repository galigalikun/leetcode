fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
}

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut result = vec![0; nums.len()];
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                if let Some(r) = result.get_mut(idx) {
                    *r = nums[i];
                }
                idx += 1;
            }
        }
        *nums = result;
    }
}
