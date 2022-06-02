fn main() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
        0
    );
}

struct Solution {}
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut product = 1;
        let mut left = 0;
        let mut ans = 0;
        for right in 0..nums.len() {
            product *= nums[right];
            while product >= k && left < nums.len() {
                product /= nums[left];
                left += 1;
            }
            ans += right as i32 - left as i32 + 1;
        }
        return if ans > 0 { ans } else { 0 };
    }
}
