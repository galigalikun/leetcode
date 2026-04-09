fn main() {
    assert_eq!(
        Solution::xor_after_queries(
            vec![1, 1, 1],
            vec![[0, 2, 1, 4]].iter().map(|v| v.to_vec()).collect()
        ),
        4
    );
    assert_eq!(
        Solution::xor_after_queries(
            vec![2, 3, 1, 5, 4],
            vec![[1, 4, 2, 3], [0, 2, 1, 2]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        31
    );
}

struct Solution;
impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        for q in queries {
            for i in q[0]..=q[1] {
                nums[i as usize] ^= q[2];
            }
        }
        nums.into_iter().fold(0, |a, b| a ^ b)
    }
}
