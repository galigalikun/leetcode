fn main() {
    assert_eq!(Solution::sum_subseq_widths(vec![2,1,3]), 6);
    assert_eq!(Solution::sum_subseq_widths(vec![2]), 0);
}

struct Solution;
impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        const MODULO: i64 = 1_000_000_007;
        let sorted = {
            let mut v = nums;
            v.sort_unstable();
            v
        };
        let n = sorted.len();
        let pow2: Vec<i64> = std::iter::successors(Some(1i64), |&p| Some(p * 2 % MODULO))
            .take(n)
            .collect();
        let sum = sorted
            .iter()
            .enumerate()
            .fold(0i64, |acc, (i, &x)| {
                (acc + x as i64 * (pow2[i] - pow2[n - 1 - i]) % MODULO) % MODULO
            });
        ((sum % MODULO + MODULO) % MODULO) as i32
    }
}
