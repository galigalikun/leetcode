fn main() {
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
}

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for n in nums.clone() {
            count += nums.iter().filter(|&x| *x == n).count() as i32 - 1;
        }
        return count / 2;
    }
}
