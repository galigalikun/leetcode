fn main() {
    assert_eq!(
        Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2),
        true
    );
    assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
}

struct Solution;
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = -k - 1;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                if i as i32 - last <= k {
                    return false;
                }
                last = i as i32;
            }
        }
        return true;
    }
}
