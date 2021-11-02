fn main() {
    assert_eq!(
        Solution::min_distance("horse".to_string(), "ros".to_string()),
        3
    );
    assert_eq!(
        Solution::min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}

pub struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut result = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            result[i][0] = i;
        }
        for i in 0..=n {
            result[0][i] = i;
        }
        for i in 1..=m {
            for j in 1..=n {
                if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                    result[i][j] = result[i - 1][j - 1];
                } else {
                    result[i][j] = 1 + std::cmp::min(
                        result[i - 1][j - 1],
                        std::cmp::min(result[i - 1][j], result[i][j - 1]),
                    );
                }
            }
        }
        return result[m][n] as i32;
    }
}
