fn main() {
    assert_eq!(Solution::minimum_distance("CAKE".to_string()), 3);
    assert_eq!(Solution::minimum_distance("HAPPY".to_string()), 6);
    assert_eq!(Solution::minimum_distance("NEW".to_string()), 3);
    assert_eq!(Solution::minimum_distance("YEAR".to_string()), 7);
    assert_eq!(Solution::minimum_distance("AA".to_string()), 0);
    assert_eq!(Solution::minimum_distance("AB".to_string()), 1);
    assert_eq!(Solution::minimum_distance("ABCA".to_string()), 2);
    assert_eq!(Solution::minimum_distance("ABAC".to_string()), 2);
    assert_eq!(Solution::minimum_distance("ABACA".to_string()), 3);
    assert_eq!(Solution::minimum_distance("ABACAB".to_string()), 4);
    assert_eq!(Solution::minimum_distance("ABACABA".to_string()), 5);
    assert_eq!(Solution::minimum_distance("ABACABAC".to_string()), 6);
}

struct Solution;
impl Solution {
    fn dist(a: usize, b: usize) -> i32 {
        if a == 26 || b == 26 {
            return 0;
        }
        let x = a / 6;
        let y = a % 6;
        let x2 = b / 6;
        let y2 = b % 6;
        return ((x as i32 - x2 as i32).abs() + (y as i32 - y2 as i32).abs()) as i32;
    }
    pub fn minimum_distance(word: String) -> i32 {
        let mut dp = vec![vec![vec![0; 27]; 27]; word.len() + 1];
        let mut ans = 0;
        for i in (0..word.len()).rev() {
            let c = word.chars().nth(i).unwrap() as usize - 'A' as usize;
            for j in 0..27 {
                for k in 0..27 {
                    dp[i][j][k] = std::cmp::min(
                        dp[i + 1][c][k] + Self::dist(j, c),
                        dp[i + 1][j][c] + Self::dist(k, c),
                    );
                }
            }
            ans += dp[i][26][26];
        }
        return ans;
    }
}
