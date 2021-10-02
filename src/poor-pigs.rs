fn main() {
    assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
    assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
    assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
}

pub struct Solution {}
// https://dreamume.medium.com/leetcode-458-poor-pigs-adc1bef981c1
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let times = minutes_to_test / minutes_to_die + 1;
        let mut result = 0;
        while times.pow(result) < buckets {
            result += 1;
        }
        return result as i32;
    }
}
