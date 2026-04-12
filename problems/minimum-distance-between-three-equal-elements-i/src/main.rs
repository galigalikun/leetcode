fn main() {
    assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    assert_eq!(Solution::minimum_distance(vec![1]), -1);
}

struct Solution;
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    ans = ans.min((j - i) as i32);
                }
            }
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}
