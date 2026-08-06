fn main() {
    assert_eq!(Solution::consecutive_numbers_sum(5), 2);
    assert_eq!(Solution::consecutive_numbers_sum(9), 3);
    assert_eq!(Solution::consecutive_numbers_sum(15), 4);
}

struct Solution {}
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let n = n as i64;
        let mut count = 0;
        let mut k = 1i64;

        while k * (k - 1) / 2 < n {
            let remaining = n - k * (k - 1) / 2;
            if remaining % k == 0 {
                count += 1;
            }
            k += 1;
        }

        count
    }
}
