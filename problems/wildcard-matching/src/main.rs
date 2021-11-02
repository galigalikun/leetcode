fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    assert_eq!(
        Solution::is_match("cb".to_string(), "?a".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match("adceb".to_string(), "*a*b".to_string()),
        true
    );
    assert_eq!(
        Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
        false
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/wildcard-pattern-matching/
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let n = s.len();
        let m = p.len();

        // empty pattern can only match with
        // empty string
        if n == 0 && m == 0 {
            return true;
        }

        // lookup table for storing results of
        // initialize lookup table to false
        let mut lookup = vec![vec![false; m + 1]; n + 1];

        // empty pattern can match with empty string
        lookup[0][0] = true;

        // Only '*' can match with empty string
        for j in 1..=m {
            if Some('*') == p.chars().nth(j - 1) {
                lookup[0][j] = lookup[0][j - 1];
            }
        }

        // fill the table in bottom-up fashion
        for i in 1..=n {
            for j in 1..=m {
                if Some('*') == p.chars().nth(j - 1) {
                    // Two cases if we see a '*'
                    // a) We ignore '*'' character and move
                    //    to next  character in the pattern,
                    //     i.e., '*' indicates an empty
                    //     sequence.
                    // b) '*' character matches with ith
                    //     character in input
                    lookup[i][j] = lookup[i][j - 1] || lookup[i - 1][j];
                } else if Some('?') == p.chars().nth(j - 1)
                    || s.chars().nth(i - 1) == p.chars().nth(j - 1)
                {
                    // Current characters are considered as
                    // matching in two cases
                    // (a) current character of pattern is '?'
                    // (b) characters actually match
                    lookup[i][j] = lookup[i - 1][j - 1];
                } else {
                    // If characters don't match
                    lookup[i][j] = false;
                }
            }
        }
        return lookup[n][m];
    }
}
