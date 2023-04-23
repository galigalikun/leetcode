fn main() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec![4,2,3], 1), 5);
    assert_eq!(Solution::largest_sum_after_k_negations(vec![3,-1,0,2], 3), 6);
    assert_eq!(Solution::largest_sum_after_k_negations(vec![2,-3,-1,5,-4], 2), 13);
}

struct Solution;
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut k = k;
        let mut i = 0;
        while k > 0 && i < nums.len() && nums[i] < 0 {
            nums[i] = -nums[i];
            i += 1;
            k -= 1;
        }
        println!("debug {:?}", nums);
        return 0;
    }
}
