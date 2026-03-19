fn main() {
    assert_eq!(
        Solution::pyramid_transition(
            "BCD".to_string(),
            vec![
                "BCC".to_string(),
                "CDE".to_string(),
                "CEA".to_string(),
                "FFF".to_string()
            ]
        ),
        true
    );
    assert_eq!(
        Solution::pyramid_transition(
            "AAAA".to_string(),
            vec![
                "AAB".to_string(),
                "AAC".to_string(),
                "BCD".to_string(),
                "BBE".to_string(),
                "DEF".to_string()
            ]
        ),
        false
    );
    assert_eq!(
        Solution::pyramid_transition(
            "ABC".to_string(),
            vec![
                "ABD".to_string(),
                "ABE".to_string(),
                "BCF".to_string(),
                "BCA".to_string(),
                "DAC".to_string(),
                "EFC".to_string(),
            ]
        ),
        true
    );
}

struct Solution {}
impl Solution {
    fn dfs(bottom: &[char], allowed: &[Vec<char>]) -> bool {
        if bottom.len() == 1 {
            return true;
        }
        let mut next = Vec::new();
        for i in 0..bottom.len() - 1 {
            let mut found = false;
            for a in allowed {
                if a[0] == bottom[i] && a[1] == bottom[i + 1] {
                    next.push(a[2]);
                    found = true;
                } else if a[0] > bottom[i] || (a[0] == bottom[i] && a[1] > bottom[i + 1]) {
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        Self::dfs(&next, allowed)
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let bottom = bottom.chars().collect::<Vec<char>>();
        let mut allowed = allowed
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        allowed.sort_unstable();
        Self::dfs(&bottom, &allowed)
    }
}
