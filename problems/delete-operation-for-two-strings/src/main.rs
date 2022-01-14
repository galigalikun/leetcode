fn main() {
    assert_eq!(
        Solution::min_distance("sea".to_string(), "eat".to_string()),
        2
    );
    assert_eq!(
        Solution::min_distance("leetcode".to_string(), "etco".to_string()),
        4
    );
    assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0);
    assert_eq!(Solution::min_distance("a".to_string(), "ab".to_string()), 1);
}

// https://dev.to/seanpgallivan/solution-delete-operation-for-two-strings-235k
struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut m = word1.len();
        let mut n = word2.len();
        let mut w1 = word1.clone();
        let mut w2 = word2.clone();
        if m < n {
            n = word1.len();
            m = word2.len();
            w1 = word2;
            w2 = word1;
        }
        let mut dp_last = vec![0; n + 1];
        let mut dp_curr = vec![0; n + 1];
        for i in 0..m {
            for j in 0..n {
                dp_curr[j + 1] = if w1.chars().nth(i) == w2.chars().nth(j) {
                    dp_last[j] + 1
                } else {
                    std::cmp::max(dp_curr[j], dp_last[j + 1])
                };
            }
            let a = dp_curr;
            dp_curr = dp_last;
            dp_last = a;
        }
        return (m + n - 2 * dp_last[n]) as i32;
    }
}
