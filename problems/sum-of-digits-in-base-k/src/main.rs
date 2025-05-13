fn main() {
    assert_eq!(Solution::sum_base(34, 6), 9);
    assert_eq!(Solution::sum_base(10, 10), 1);
}

struct Solution;
impl Solution {
    fn sum_base_helper(n: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut n = n;
        while n > 0 {
            sum += n % k;
            n /= k;
        }
        sum
    }
    pub fn sum_base(n: i32, k: i32) -> i32 {
        return Self::sum_base_helper(n, k);
    }
}
