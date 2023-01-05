fn main() {
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["e".to_string(), "o".to_string()]
        ),
        vec!["facebook", "google", "leetcode"]
    );
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["l".to_string(), "e".to_string()]
        ),
        vec!["apple", "google", "leetcode"]
    );
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["lo".to_string(), "eo".to_string()]
        ),
        vec!["google", "leetcode"]
    );
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["e".to_string(), "oo".to_string()]
        ),
        vec!["facebook", "google"]
    );
}

struct Solution;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        for w1 in words1 {
            let mut b = true;
            for w2 in words2.clone() {
                for c2 in w2.chars() {
                    if w1.find(c2) == None {
                        b = false;
                        break;
                    }
                }
                if !b {
                    break;
                }
            }
            if b {
                ans.push(w1);
            }
        }
        return ans;
    }
}
