fn main() {
    assert_eq!(Solution::two_egg_drop(2), 2);
    assert_eq!(Solution::two_egg_drop(100), 14);
}

struct Solution;
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        return if n <= 2 {
            n
        } else {
            let mut dp = vec![0; (n + 1) as usize];
            for i in 1..=n {
                dp[i as usize] = i;
                for j in 1..i {
                    let worst_case = 1 + dp[j as usize - 1] + dp[i as usize - j as usize];
                    if worst_case < dp[i as usize] {
                        dp[i as usize] = worst_case;
                    }
                }
            }
            dp[n as usize]
        };
    }
}
