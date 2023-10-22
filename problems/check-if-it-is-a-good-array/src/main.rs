fn main() {
    assert_eq!(Solution::is_good_array(vec![12, 5, 7, 23]), true);
    assert_eq!(Solution::is_good_array(vec![29, 6, 10]), true);
    assert_eq!(Solution::is_good_array(vec![3, 6]), false);
}

struct Solution;
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Self::gcd(b, a % b);
    }
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let mut gcd = nums[0];
        for i in 1..nums.len() {
            gcd = Self::gcd(gcd, nums[i]);
        }
        return gcd == 1;
    }
}
