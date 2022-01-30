fn main() {
    assert_eq!(Solution::find_integers(5), 5);
    assert_eq!(Solution::find_integers(1), 2);
    assert_eq!(Solution::find_integers(2), 3);
    assert_eq!(Solution::find_integers(6), 5);
}

struct Solution {}
// https://github.com/cherryljr/LeetCode/blob/master/Non-negative%20Integers%20without%20Consecutive%20Ones.java
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        match n {
            0 | 1 => {
                return n + 1;
            }
            _ => {
                let b = format!("{:b}", n);
                let mut f = vec![0; b.len()];
                f[0] = 1;
                f[1] = 2;
                for i in 2..b.len() {
                    f[i] = f[i - 1] + f[i - 2];
                }
                let mut ans = 0;
                for i in 0..b.len() {
                    if b.chars().nth(i) == Some('1') {
                        ans += f[b.len() - i - 1];
                        if i > 0 && b.chars().nth(i - 1) == Some('1') {
                            return ans;
                        }
                    }
                }
                return ans + 1;
            }
        }
    }
}
