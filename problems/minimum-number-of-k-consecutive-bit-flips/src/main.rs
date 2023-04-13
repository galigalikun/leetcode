fn main() {
    assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    assert_eq!(
        Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
        3
    );
}

struct Solution;
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut flips = 0;
        let mut flip = 0;
        let mut i = 0;
        while i < nums.len() {
            if flip % 2 == 1 {
                nums[i] = 1 - nums[i];
            }
            if nums[i] == 0 {
                if i + k as usize > nums.len() {
                    return -1;
                }
                flips += 1;
                flip += 1;
                nums[i] = 1;
            }
            i += 1;
        }
        flips
    }
}
