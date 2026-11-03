fn is_subsequence(s: &str, t: &str) -> bool {
    let mut chars = s.chars();
    let mut current = chars.next();

    for ch in t.chars() {
        if current == Some(ch) {
            current = chars.next();
        }
    }

    current.is_none()
}

fn main() {
    let result = Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string());
    assert_eq!(result, "cabac");

    let result = Solution::shortest_common_supersequence(
        "aaaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
    );
    assert_eq!(result, "aaaaaaaa");

    let result = Solution::shortest_common_supersequence("abc".to_string(), "def".to_string());
    assert_eq!(result.len(), 6);
    assert!(is_subsequence("abc", &result));
    assert!(is_subsequence("def", &result));

    let result = Solution::shortest_common_supersequence("abc".to_string(), "abc".to_string());
    assert_eq!(result, "abc");
}

struct Solution;
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let m = chars1.len();
        let n = chars2.len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        let mut i = m;
        let mut j = n;
        let mut result = Vec::new();

        while i > 0 && j > 0 {
            if chars1[i - 1] == chars2[j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] >= dp[i][j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
            } else {
                result.push(chars2[j - 1]);
                j -= 1;
            }
        }

        while i > 0 {
            result.push(chars1[i - 1]);
            i -= 1;
        }

        while j > 0 {
            result.push(chars2[j - 1]);
            j -= 1;
        }

        result.into_iter().rev().collect()
    }
}
