fn main() {
    assert_eq!(Solution::count_pairs(vec![1,4,2,7], 2, 6), 6);
    assert_eq!(Solution::count_pairs(vec![9,8,4,2,1], 5, 14), 8);
}

struct Solution;
impl Solution {
    fn count_pairs_helper(nums: &[i32], low: i32, high: i32) -> i32 {
        let mut count = 0;
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let sum = nums[i] + nums[j];
                if sum >= low && sum <= high {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        return Self::count_pairs_helper(&nums, low, high);
    }
}
