fn main() {
    assert_eq!(
        Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
        vec![2, 4, 4, 4]
    );
    assert_eq!(
        Solution::decompress_rl_elist(vec![1, 1, 2, 3]),
        vec![1, 3, 3]
    );
}

struct Solution;
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums.len() {
            if i % 2 == 0 {
                for _ in 0..nums[i] {
                    result.push(nums[i + 1]);
                }
            }
        }
        return result;
    }
}
