fn main() {
    assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
    assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
}

struct Solution {}
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut a = 0;
        let mut b = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] > nums[j] {
                    b += 1;
                }
            }
            if (i + 1) > nums.len() && nums[i] > nums[i + 1] {
                a += 1;
            }
        }
        return a == b;
    }
}
