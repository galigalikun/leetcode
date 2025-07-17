fn main() {
    assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
    assert_eq!(
        Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
        64
    );
}

struct Solution;
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        return nums.iter().cloned().max().unwrap() * nums.iter().cloned().nth(1).unwrap()
            - nums.iter().cloned().min().unwrap()
                * nums.iter().cloned().nth(nums.len() - 2).unwrap();
    }
}
