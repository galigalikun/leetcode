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
        for (i, s) in strs.into_iter().enumerate() {
            let mut a = s.chars().collect::<Vec<_>>();
            a.sort();
            let b = a.iter().collect::<String>();
            let c = a.iter().rev().collect::<String>();
            if s != b && s != c {
                ans = i as i32;
                break;
            }
        }
        return ans;
    }
}
