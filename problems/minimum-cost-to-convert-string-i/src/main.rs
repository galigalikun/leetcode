fn main() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20]
        ),
        28
    );
    assert_eq!(
        Solution::minimum_cost(
            "aaaa".to_string(),
            "bbbb".to_string(),
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2]
        ),
        12
    );
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "abce".to_string(),
            vec!['a'],
            vec!['e'],
            vec![10000]
        ),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut ans = 0;
        for (s, t) in source.chars().zip(target.chars()) {
            if s != t {
                let mut c = i64::MAX;
                for i in 0..original.len() {
                    if original[i] == s && changed[i] == t {
                        c = c.min(cost[i] as i64);
                    }
                }
                if c == i64::MAX {
                    return -1;
                }
                ans += c;
            }
        }
        ans
    }
}
