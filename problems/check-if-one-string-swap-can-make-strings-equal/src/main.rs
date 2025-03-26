fn main() {
    assert_eq!(
        Solution::are_almost_equal("bank".to_string(), "kanb".to_string()),
        true
    );
    assert_eq!(
        Solution::are_almost_equal("attack".to_string(), "defend".to_string()),
        false
    );
    assert_eq!(
        Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()),
        true
    );
    assert_eq!(
        Solution::are_almost_equal("abcd".to_string(), "dcba".to_string()),
        false
    );
    assert_eq!(
        Solution::are_almost_equal("acac".to_string(), "acbb".to_string()),
        false
    );
    assert_eq!(
        Solution::are_almost_equal("aa".to_string(), "ac".to_string()),
        false
    );
}

struct Solution;
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut diff = 0;
        let mut diff_idx = vec![];
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                diff += 1;
                diff_idx.push(i);
            }
        }
        if diff == 2 {
            let idx1 = diff_idx[0];
            let idx2 = diff_idx[1];
            if s1[idx1] == s2[idx2] && s1[idx2] == s2[idx1] {
                return true;
            }
            return false;
        } else if diff > 0 {
            return false;
        }

        return true;
    }
}
