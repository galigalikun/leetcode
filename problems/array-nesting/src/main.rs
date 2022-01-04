fn main() {
    assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
}

struct Solution {}
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut idx = 0;
        let mut ans = 1;
        loop {
            idx = nums[idx as usize];
            if idx == 0 {
                return ans;
            }
            ans += 1;
        }
    }
}
