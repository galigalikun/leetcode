fn main() {
    assert_eq!(Solution::num_tilings(3), 5);
    assert_eq!(Solution::num_tilings(1), 1);
    assert_eq!(Solution::num_tilings(4), 11);
}

struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }

        let mut f0 = 1_i64;
        let mut f1 = 1_i64;
        let mut f2 = 2_i64;

        for _ in 3..=n {
            let next = (2 * f2 + f0) % MOD;
            f0 = f1;
            f1 = f2;
            f2 = next;
        }

        f2 as i32
    }
}
