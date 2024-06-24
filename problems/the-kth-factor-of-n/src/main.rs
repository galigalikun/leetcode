fn main() {
    assert_eq!(Solution::kth_factor(12, 3), 3);
    assert_eq!(Solution::kth_factor(7, 2), 7);
    assert_eq!(Solution::kth_factor(4, 4), -1);
}

struct Solution;
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            if n % i == 0 {
                count += 1;
                if count == k {
                    return i;
                }
            }
        }
        return -1;
    }
}
