fn main() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );
    assert_eq!(
        Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.clone().len();
        let mut vec = score.clone();
        vec.sort_by(|a, b| b.cmp(a));
        let mut map = HashMap::new();
        for i in 0..vec.len() {
            map.insert(
                vec[i],
                if i == 0 {
                    "Gold Medal".to_string()
                } else if i == 1 {
                    "Silver Medal".to_string()
                } else if i == 2 {
                    "Bronze Medal".to_string()
                } else {
                    format!("{}", i + 1)
                },
            );
        }
        let mut ans = vec!["".to_string(); n];
        for i in 0..n {
            let s = score[i];
            ans[i] = map.get(&s).unwrap().to_string();
        }
        return ans;
    }
}
