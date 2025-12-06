fn main() {
    assert_eq!(Solution::count_partitions(vec![9, 4, 1, 3, 7], 4), 6);
    assert_eq!(Solution::count_partitions(vec![3, 3, 4], 0), 2);
}

struct Solution;
impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let modulo = 1_000_000_007;
        let total: i64 = nums.iter().map(|&x| x as i64).sum();
        let target = total - k as i64 + 1;
        if target <= 0 {
            return (1i64 << n) as i32 % modulo;
        }
        return 0;
    }
}
