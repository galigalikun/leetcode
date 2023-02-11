fn main() {
    assert_eq!(
        Solution::min_deletion_size(vec![
            "cba".to_string(),
            "daf".to_string(),
            "ghi".to_string()
        ]),
        1
    );
    assert_eq!(
        Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]),
        0
    );
    assert_eq!(
        Solution::min_deletion_size(vec![
            "zyx".to_string(),
            "wvu".to_string(),
            "tsr".to_string()
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut v = vec!["".to_string(); strs[0].len()];
        for (_i, s) in strs.into_iter().enumerate() {
            for (j, c) in s.chars().enumerate() {
                v[j] = format!("{}{}", v[j], c);
            }
        }

        for s in v {
            let mut a = s.chars().collect::<Vec<_>>();
            a.sort();
            if s != a.iter().collect::<String>() {
                ans += 1;
            }
        }
        return ans;
    }
}
