fn main() {
    assert_eq!(Solution::min_steps(3), 3);
    assert_eq!(Solution::min_steps(1), 0);
}

// https://twchen.gitbook.io/leetcode/dynamic-programming/2-keys-keyboard
struct Solution {}
impl Solution {
    fn helper(n: i32, m: i32) -> i32 {
        if n < m {
            return 1000;
        } else if n > m {
            return Solution::helper(n - m, m) + 1;
        } else {
            return Solution::min_steps(n) + 1;
        }
    }
    pub fn min_steps(n: i32) -> i32 {
        // A 1 -> 0
        // AA 2 -> 2
        // AAA 3 -> 3
        // AAAA 4 -> 4
        // AAAAA 5 -> 5
        // AAAAAA 6 -> 5
        // AAAAAAA 7 -> 7
        // AAAAAAAA 8 -> 6
        // AAAAAAAAA 9 -> 6
        // AAAAAAAAAA 10 -> 7
        // AAAAAAAAAAA 11 -> 11
        // AAAAAAAAAAAA 12 -> 7 3*3+3
        if n == 1 {
            return 0;
        }
        let mut ans = 1000;
        let mut m = 1;
        loop {
            let step = Solution::helper(n - m, m);
            ans = std::cmp::min(ans, step);
            m += 1;
            if 2 * m > n {
                break;
            }
        }
        return ans + 1;
    }
}
