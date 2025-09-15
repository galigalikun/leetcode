fn main() {
    assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
    assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8]), 1);
    assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
}

struct Solution;
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        return gcd(*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
