fn main() {
    assert_eq!(Solution::num_squareful_perms(vec![1,17,8]), 2);
    assert_eq!(Solution::num_squareful_perms(vec![2,2,2]), 1);
}

struct Solution;
impl Solution {
    fn is_squareful(a: i32, b: i32) -> bool {
        let c = (a as f64 + b as f64).sqrt();
        return c == c.floor();
    }
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if Self::is_squareful(nums[i], nums[j]) {
                    ans += 1;
                }
            }
        }
        return ans;
    }
}
