fn main() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
    assert_eq!(Solution::super_egg_drop(2, 6), 3);
    assert_eq!(Solution::super_egg_drop(3, 14), 4);
}

struct Solution;
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        if n <= 0 || k <= 0 {
            return 0;
        }

        let eggs = k as usize;
        let target = n as i64;

        // dp[e] = max floors testable with current moves and e eggs.
        let mut dp = vec![0_i64; eggs + 1];
        let mut moves = 0_i32;

        while dp[eggs] < target {
            moves += 1;
            for e in (1..=eggs).rev() {
                dp[e] = dp[e] + dp[e - 1] + 1;
            }
        }

        moves
    }
}
