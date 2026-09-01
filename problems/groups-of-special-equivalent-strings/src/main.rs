fn main() {
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abcd".to_string(),
            "cdab".to_string(),
            "cbad".to_string(),
            "xyzz".to_string(),
            "zzxy".to_string(),
            "zzyx".to_string()
        ]),
        3
    );
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abc".to_string(),
            "acb".to_string(),
            "bac".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "cba".to_string()
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut groups = HashSet::new();

        for word in words {
            let mut even = Vec::new();
            let mut odd = Vec::new();

            for (i, ch) in word.chars().enumerate() {
                if i % 2 == 0 {
                    even.push(ch);
                } else {
                    odd.push(ch);
                }
            }

            even.sort_unstable();
            odd.sort_unstable();

            groups.insert((even, odd));
        }

        groups.len() as i32
    }
}
