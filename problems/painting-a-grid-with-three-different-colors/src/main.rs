fn main() {
    assert_eq!(Solution::color_the_grid(1, 1), 3);
    assert_eq!(Solution::color_the_grid(1, 2), 6);
    assert_eq!(Solution::color_the_grid(5, 5), 580986);
}

struct Solution;
impl Solution {
    const MOD: i32 = 1_000_000_007;
    fn color_the_grid_helper(m: i32, n: i32, prev1: i32, prev2: i32, col: i32) -> i32 {
        if col == n {
            return 1;
        }
        let mut res = 0;
        for c1 in 0..3 {
            if c1 == prev1 || c1 == prev2 {
                continue;
            }
            for c2 in 0..3 {
                if c2 == c1 || c2 == prev1 || c2 == prev2 {
                    continue;
                }
                res = (res + Self::color_the_grid_helper(m, n, c1, c2, col + 1)) % Self::MOD;
            }
        }
        res
    }
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        return Self::color_the_grid_helper(m, n, 0, 0, 0);
    }
}
